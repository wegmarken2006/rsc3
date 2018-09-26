use sc3::*;
use osc::{sc_play, sc_play_vec};

pub struct Oscillator {
    name: String,
    freq: f32,
    phase: f32,
    rate: Rate,
    add: Ugen,
    mul: Ugen,
}

impl Oscillator {
    pub fn new(name: &str, freq: f32, phase: f32) -> Oscillator {
        let osc = Oscillator {
            name: name.to_string(),
            freq: freq,
            phase: phase,
            rate: Rate::RateAr,
            add: Ugen::FConst(FConst { value: 0.0 }),
            mul: Ugen::FConst(FConst { value: 0.0 }),
        };
        osc
    }
    pub fn rate(mut self, rate: Rate) -> Oscillator {
        self.rate = rate;
        self
    }
    pub fn run(self, ou: i32) -> Ugen {
        let inputs = const_vec(vec![self.freq, self.phase]);
        let osc = mk_oscillator(self.rate, &self.name, inputs, ou);
        osc
    }
}

fn const_vec(nums: Vec<f32>) -> UgenList {
    let mut out = Vec::new();
    for elem in nums {
        out.push(Box::new(Ugen::FConst(FConst{value: elem})));
    }
    out
}

pub fn sin_osc(freq: f32, phase: f32) -> Ugen {
    let osc = Oscillator::new("SinOsc", freq, phase);
    osc.run(1)
}

fn iconst_list(val: i32) -> UgenList {
    let mut out = Vec::new();
    out.push(Box::new(Ugen::IConst(IConst { value: val })));
    out
}


pub fn one_pole(ugen: Ugen, coef: f32) -> Ugen {
    return mk_filter("OnePole", vec![Box::new(ugen.clone()), Box::new(Ugen::FConst(FConst{value: coef}))], 1);
}
/*
(define one-pole
  (lambda (input coef)
    (mk-ugen (list "OnePole" (list 0) (list input coef) nil 1 nil nil))))

(define construct-ugen
  (lambda (name rate inputs mce outputs special id)
*/

pub fn out(a: i32, ugen: &Ugen) -> Ugen {
    return mk_filter_mce("Out", iconst_list(a), ugen, 0);
}
    
pub fn brown_noise () -> Ugen {
    return mk_osc_id(Rate::RateAr, "BrownNoise", vec![], 1);
}

pub fn lpf(ugen: Ugen, freq: f32) -> Ugen {
    return mk_filter("LPF", vec![Box::new(ugen.clone()), Box::new(Ugen::FConst(FConst{value: freq}))], 1);
}
/*
(define lpf
  (lambda (input freq)
    (mk-ugen (list "LPF" (list 0) (list input freq) nil 1 nil nil))))
*/

pub fn rhpf(ugen1: Ugen, ugen2: Ugen, coef: f32) -> Ugen {
    return mk_filter("RHPF", vec![Box::new(ugen1.clone()), Box::new(ugen2.clone()), 
    Box::new(Ugen::FConst(FConst{value: coef}))], 1);
}
    

use std::any::Any;

pub fn add<T: Any, U: Any>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x + y}, op1, op2);
}

/*
pub fn mul<T: 'static, U: 'static>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x * y}, op1, op2);
}
*/
pub fn mul<T: Any, U: Any>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x * y}, op1, op2);
}

pub fn sub<T: Any, U: Any>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x - y}, op1, op2);
}

pub fn play_demo_1() {
    sc_play(&sin_osc(440.0, 0.0));
}

pub fn play_demo_2() {
    let ug0 = mul(rhpf(one_pole(brown_noise(), 0.99), add(mul(lpf(brown_noise(), 14.0), 400.0), 500.0), 0.03), 1.003);
    let ug1 = mul(rhpf(one_pole(brown_noise(), 0.99), add(mul(lpf(brown_noise(), 20.0), 800.0), 1000.0), 0.03), 1.005);

    let ug2 = mul(4.0, add(ug0, ug1));
    sc_play_vec(vec![ug2.clone(), ug2]);
    //sc_play_vec(vec![mul(sin_osc(440.0, 0.0), 0.1), mul(sin_osc(100.0, 0.0), 0.1)]);
}

/*
{
({RHPF.ar(OnePole.ar(BrownNoise.ar, 0.99), LPF.ar(BrownNoise.ar, 14)
* 400 + 500, 0.03, 0.003)}!2)
+ ({RHPF.ar(OnePole.ar(BrownNoise.ar, 0.99), LPF.ar(BrownNoise.ar, 20)
* 800 + 1000, 0.03, 0.005)}!2)
* 4
}.play
*/


macro_rules! osc_m {
    ($name: expr, $first: expr, $second: expr) => {
        Oscillator::new($name, $first, $second).run(1) 
    };
    ($name: expr, $first: expr, $second: expr, rate: $third: expr) => {
        Oscillator::new($name, $first, $second).rate($third).run(1)
    };
}


macro_rules! sin_osc_m {
    ($first: expr, $second: expr) => {
        osc_m!("SinOsc", $first, $second) 
    };
    ($first: expr, $second: expr, rate: $third: expr) => {
        osc_m!("SinOsc", $first, $second, rate: $third)
    };
}


#[test]
fn test2() {
    assert_eq!(true, true);
}

