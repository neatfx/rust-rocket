fn main() {
    let err = network_rocket::rocket_ins().launch();

    println!("ROCKET DIDN'T LAUNCH!");
    println!("LAUNCH ERROR: {}", err);
}
