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
        let mut osc = Oscillator {
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
    pub fn run(mut self, ou: i32) -> Ugen {
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


pub fn one_pole(ugen: &Ugen, coef: f32) -> Ugen {
    return mk_filter("OnePole", vec![Box::new(ugen.clone()), Box::new(Ugen::FConst(FConst{value: coef}))], 1, 0);
}


pub fn out(a: i32, ugen: &Ugen) -> Ugen {
    return mk_filter_mce("Out", iconst_list(a), ugen, 0);
}
    
pub fn brown_noise () -> Ugen {
    return mk_osc_id(Rate::RateAr, "BrownNoise", vec![], 1);
}

pub fn lpf(ugen: &Ugen, coef: f32) -> Ugen {
    return mk_filter("LPF", vec![Box::new(ugen.clone()), Box::new(Ugen::FConst(FConst{value: coef}))], 1, 0);
}
    

pub fn rhpf(ugen1: &Ugen, ugen2: &Ugen, coef: f32) -> Ugen {
    return mk_filter("RHPF", vec![Box::new(ugen1.clone()), Box::new(ugen2.clone()), 
    Box::new(Ugen::FConst(FConst{value: coef}))], 1, 0);
}
    


pub fn add<T: 'static, U: 'static>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x + y}, op1, op2);
}

pub fn mul<T: 'static, U: 'static>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x * y}, op1, op2);
}

pub fn sub<T: 'static, U: 'static>(op1: T, op2: U) -> Ugen {
    return mk_binary_operator(0, |x, y| {x - y}, op1, op2);
}

pub fn play_demo_1() {
    sc_play(&sin_osc(440.0, 0.0));
}

pub fn play_demo_2() {
    let ug20 = one_pole(&brown_noise(), 0.99);
    let ug21 = lpf(&brown_noise(), 14.0);
    let ug210 = mul(&ug21, 400.0);
    let ug22 = add(&ug210, 500.0);
    let ug220 = rhpf(&ug20, &ug22, 0.03);
    let ug100 = mul(&ug220, 0.03);

    sc_play_vec(vec![ug100.clone(), ug100]);
    //sc_play_vec(vec![mul(sin_osc(440.0, 0.0), 0.1), mul(sin_osc(100.0, 0.0), 0.1)]);
}


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

