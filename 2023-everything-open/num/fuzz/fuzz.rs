extern crate num;

use std::str::FromStr;

use num::Num;

fn do_test(data: &[u8]) {
    let res = String::from_utf8(data.to_vec());
    if let Ok(s) = res {
        let got = Num::from_str(&s);
        match i32::from_str_radix(&s, 10) {
            Err(_) => assert!(got.is_err()),
            Ok(n) => {
                let got = got.expect("failed to parse valid string");
                assert_eq!(Num::from_signed(n), got);
            }
        }
    }
}

#[macro_use]
extern crate honggfuzz;

fn main() {
    loop {
        fuzz!(|d| { do_test(d) });
    }
}
