#![feature(proc_macro_non_items, proc_macro_gen, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust 2018!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
