use super::super::state::AssetsDir;
use rocket::fairing::AdHoc;

pub fn adhoc_launch_printer() -> AdHoc {
    AdHoc::on_launch("Printer", |_| {
        println!("Rocket is about to launch! Exciting! Here we go...");
    })
}

pub fn adhoc_put_rewriter() -> AdHoc {
    AdHoc::on_request("All Rewriter", |_, _| {
        println!("Rocket is requesting...");
    })
}

pub fn adhoc_assets_dir_config() -> AdHoc {
    AdHoc::on_attach("Assets Dir Config", |rocket| {
        let assets_dir = rocket.config().get_str("assets_dir").unwrap().to_string();

        println!("Managed state AssetsDir added : {}", assets_dir);
        Ok(rocket.manage(AssetsDir(assets_dir)))
    })
}
