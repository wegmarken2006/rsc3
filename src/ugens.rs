use sc3::*;

struct Oscillator {
    name: String,
    freq: f32,
    phase: f32,
    rate: Rate,
    add: Ugen,
    mul: Ugen,
}

impl Oscillator {
    fn new(name: &str, freq: f32, phase: f32) -> Oscillator {
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
    fn rate(mut self, rate: Rate) -> Oscillator {
        self.rate = rate;
        self
    }
    fn run(mut self, ou: i32) -> Ugen {
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


#[test]
fn test2() {
    assert_eq!(true, true);
}

