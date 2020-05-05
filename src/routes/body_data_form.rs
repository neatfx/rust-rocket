use rocket::http::RawStr;
use rocket::request::{Form, FormDataError, FormError, FromFormValue};
use rocket::response::Redirect;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![login, user_page, sink]
}

#[derive(Debug)]
struct StrongPassword<'r>(&'r str);

#[derive(Debug)]
struct AdultAge(isize);

#[derive(FromForm)]
struct UserLogin<'r> {
    username: &'r RawStr,
    password: Result<StrongPassword<'r>, &'static str>,
    age: Result<AdultAge, &'static str>,
}

impl<'v> FromFormValue<'v> for StrongPassword<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        if v.len() < 8 {
            Err("too short!")
        } else {
            Ok(StrongPassword(v.as_str()))
        }
    }
}

impl<'v> FromFormValue<'v> for AdultAge {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let age = match isize::from_form_value(v) {
            Ok(v) => v,
            Err(_) => return Err("value is not a number."),
        };

        match age > 20 {
            true => Ok(AdultAge(age)),
            false => Err("must be at least 21."),
        }
    }
}

#[post("/body-data/form/login", data = "<user>")]
fn login(user: Form<UserLogin>) -> Result<Redirect, String> {
    if let Err(e) = user.age {
        return Err(format!("Age is invalid: {}", e));
    }

    if let Err(e) = user.password {
        return Err(format!("Password is invalid: {}", e));
    }

    if user.username == "Sergio" {
        if let Ok(StrongPassword("password")) = user.password {
            Ok(Redirect::to("/body-data/form/user/Sergio"))
        } else {
            Err("Wrong password!".to_string())
        }
    } else {
        Err(format!("Unrecognized user, '{}'.", user.username))
    }
}

#[get("/body-data/form/user/<username>")]
fn user_page(username: &RawStr) -> String {
    format!("This is {}'s page.", username)
}

/// 字段重命名 & 枚举实现 FromFormValue 特质

#[derive(Debug, FromFormValue)]
enum FormOption {
    A,
    B,
    C,
}

#[derive(Debug, FromForm)]
struct FormInput<'r> {
    checkbox: bool,
    number: usize,
    #[form(field = "type")]
    radio: FormOption,
    password: &'r RawStr,
    #[form(field = "textarea")]
    text_area: String,
    select: FormOption,
}

#[post("/body-data/form", data = "<sink>")]
fn sink(sink: Result<Form<FormInput>, FormError>) -> String {
    match sink {
        Ok(form) => format!("{:?}", &*form),
        Err(FormDataError::Io(_)) => format!("Form input was invalid UTF-8."),
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            format!("Invalid form input: {}", f)
        }
    }
}
