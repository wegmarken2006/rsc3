use sc3::*;



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


macro_rules! osc_m {
    ($name: expr, $first: expr, $second: expr) => {
        let osc = Oscillator::new($name, $first, $second).osc.run(1) 
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

