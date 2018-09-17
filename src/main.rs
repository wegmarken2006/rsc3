mod osc;
#[macro_use]
mod ugens;
mod sc3;
mod gui;
//#![allow(dead_code)]

extern crate iui;

use osc::*;
use ugens::*;
use sc3::*;
use gui::*;


//////
use std::f64;
fn main() {
    println!("start");

    run_gui();

    println!("\nend");
    
    /*
    let ug1 = synthdef("anonymous", &sin_osc(440.0, 0.0));
    print_bytes("final", &ug1);

    let ug2 = synthdef("anonymous", &sin_osc_m!(440.0, 0.0, rate: Rate::RateIr));
    print_bytes("final macro", &ug2);
*/
    //sc_start();

    
    

}

#[test]
fn test0() {
    assert_eq!(true, true);
}
