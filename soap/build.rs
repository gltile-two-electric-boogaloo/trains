use std::env;
use savon::gen;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    gen_write("./assets/example.wsdl", &out_dir).unwrap();
}