extern crate factorial;
use std::str::FromStr;
use std::env;
fn main() {
    match env::args().nth(1) {
        Option::Some(p) => {
            match i64::from_str(&p) {
                Result::Ok(n) => println!("{}", factorial::factorial(n)),
                Result::Err(e) => panic!("The parameter is invalid! {:?}", e),
            }
        }
        Option::None => panic!("None parameter provided"),
    }
}
