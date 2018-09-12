mod osc;
mod ugens;
mod sc3;

use osc::*;
use ugens::*;
use sc3::*;

//////
fn main() {
    println!("start");
    let ug1 = synthdef("anonymous", &sin_osc(440.0, 0.0));
    print_bytes("final", ug1);

    println!("\nend");
}

#[test]
fn test0() {
    assert_eq!(true, true);
}
