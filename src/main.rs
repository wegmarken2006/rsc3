mod osc;
mod ugens;
mod sc3;

use osc::*;
use ugens::*;
use sc3::*;

//////
fn main() {
    println!("start");
    let ug1 = synthdef("anonymous".to_string(), &sin_osc(440.0, 0.0));

    println!("end");
}

#[test]
fn test0() {
    assert_eq!(true, true);
}
