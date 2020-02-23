
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod utils;
use utils::math_Utils::square;
use crate::utils::math_Utils::applyToVec;

fn sum2(x: i32) -> i32 {
    return x+2;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
fn main() {
    println!("Hello");
    let v = square(23);
    println!("Square is {}",v);
    let v2 = applyToVec(vec![1, 2, 3],sum2);
    println!("Square is {:?}",v2);
    rocket::ignite().mount("/", routes![index]).launch();
}
