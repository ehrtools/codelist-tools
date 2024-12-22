use extendr_api::prelude::*;

#[extendr]
fn hello() -> &'static str {
    println!("hello function called");
    "hello"
}

extendr_module! {
    mod codelist;
    fn hello;
}
