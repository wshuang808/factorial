extern crate factorial;
use std::str::FromStr;
use std::env;
fn main() {
    match env::args().nth(1) {
        Option::Some(p) => {
            match i64::from_str(&p) {
                Result::Ok(n) if n >= 0 => println!("{}", factorial::exec(n)),
                Result::Err(e) => println!("Invalid parameter! {:?}", e),
                Result::Ok(_) => println!("Prameter must not be negavite!"),
            }
        }
        Option::None => println!("Missing parameter!"),
    }
}
