#[derive(Clone, Copy, Debug)]
enum Rate {
    RateIr = 0,
    RateKr = 1,
    RateAr = 2,
    RateDr = 3,
}

//type UgenList<'a> = &'a[Box<Ugen>];
type UgenList = Vec<Box<Ugen>>;
type RateList = Vec<Rate>;

#[derive(Clone, Debug)]
struct IConst {
    value: i32,
}
#[derive(Clone, Debug)]
struct FConst {
    value: f32,
}
#[derive(Clone, Debug)]
struct Control {
    name: String,
    index: i32,
    rate: Rate,
}
#[derive(Clone, Debug)]
struct Primitive {
    name: String,
    inputs: UgenList,
    outputs: RateList,
    special: i32,
    index: i32,
    rate: Rate,
}
#[derive(Clone, Debug)]
struct Mce {
    ugens: Vec<Box<Ugen>>,
}
#[derive(Clone, Debug)]
struct Mrg {
    left: Box<Ugen>,
    right: Box<Ugen>,
}
#[derive(Clone, Debug)]
struct Proxy {
    primitive: Primitive,
    index: i32,
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive {
            name: "".to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            special: 0,
            index: 0,
            rate: Rate::RateKr,
        }
    }
}

#[derive(Clone, Debug)]
enum Ugen {
    IConst(IConst),
    FConst(FConst),
    Control(Control),
    Primitive(Primitive),
    Mce(Mce),
    Mrg(Mrg),
    Proxy(Proxy),
}

static mut G_NEXT_ID: i32 = 0;

fn next_uid() -> i32 {
    unsafe {
        G_NEXT_ID = G_NEXT_ID + 1;
        G_NEXT_ID
    }
}

fn print_ugen(ugen: &Ugen) {
    match ugen {
        Ugen::IConst(iconst) => println!("I Value: {}", iconst.value),
        Ugen::FConst(fconst) => println!("F Value: {}", fconst.value),
        Ugen::Control(control) => println!("K Name: {}", control.name),
        Ugen::Mce(mce) => println!("MC Ulen: {}", mce.ugens.len()),
        Ugen::Mrg(mrg) => println!("MG "),
        Ugen::Primitive(primitive) => println!(
            "P Name: {} IL:{}, OL:{}",
            primitive.name,
            primitive.inputs.len(),
            primitive.outputs.len()
        ),
        Ugen::Proxy(proxy) => println!("Px Name:{}", proxy.primitive.name),
    }
}

fn print_ugens(ugens: UgenList) {
    for ugen in ugens {
        print_ugen(&ugen);
    }
}

fn iota(n: i32, init: i32, step: i32) -> Vec<i32> {
    if n == 0 {
        Vec::new()
    } else {
        let mut out = Vec::new();
        out.push(init);
        out.extend(iota(n - 1, init + step, step));
        out
    }
}

fn extend(ugens: UgenList, new_len: i32) -> UgenList {
    let ln = ugens.len() as i32;
    let mut out: UgenList = Vec::new();
    if ln > new_len {
        out.extend_from_slice(&ugens[0..new_len as usize]);
        out
    } else {
        out.extend(ugens.clone());
        out.extend(ugens);
        extend(out.clone(), new_len)
    }
}

fn rate_id(rate: Rate) {
    rate as i32;
}

fn is_sink(ugen: Ugen) -> bool {
    match ugen {
        Ugen::Mce(mce) => {
            let mut ret = false;
            for elem in mce.ugens {
                if is_sink(*elem) {
                    ret = true;
                    break;
                }
            }
            ret
        },
        Ugen::Mrg(mrg) => {
            if is_sink(*mrg.left) {
                true
            } else {
                false
            }
        },
        Ugen::Primitive(primitive) => {
            if primitive.inputs.len() == 0 {
                true
            } else {
                false
            }
        },
        _ => false,
    }
}

fn max_num(nums: Vec<i32>, start: i32 ) -> i32 {
    let mut max = start;
    for elem in nums {
        if elem > max {
            max = elem
        }
    }
    max
}

fn max_rate(rates: RateList, start: Rate ) -> Rate {
    let mut max = start;
    for elem in rates {
        if elem as i32 > max as i32{
            max = elem;
        }
    }
    max
}

fn rate_of(ugen: Ugen) -> Rate {
    match ugen {
        Ugen::Control(control) => control.rate,
        Ugen::Mce(mce) => {
            let mut rates = Vec::new();
            for ugen in mce.ugens {
                rates.push(rate_of(*ugen));
            }
            max_rate(rates, Rate::RateKr)
        },        
        Ugen::Mrg(mrg) => rate_of(*mrg.left),
        Ugen::Primitive(primitive) => primitive.rate,
        Ugen::Proxy(proxy) => proxy.primitive.rate,
        _ => Rate::RateKr
    }
}

//////
fn main() {
    println!("start");
    let o1 = iota(4, 1, 2);
    let o2 = vec![1, 3, 5, 7];
    //let ci1 = Ugen::IConst{value: 1};
    let ci1 = Ugen::IConst(IConst { value: 1 });
    let cf1 = Ugen::FConst(FConst { value: 3.3 });
    let mut ugens1: UgenList = Vec::new();
    ugens1.push(Box::new(ci1.clone()));
    ugens1.push(Box::new(cf1.clone()));
    /*
    let p1 = Ugen::Primitive(Primitive {
        name: "P1".to_string(),
        ..Primitive::default()
    });
    print_ugen(&p1);
    */
    
    println!("end");
}

#[test]
fn test1() {
    let o1 = iota(4, 1, 2);
    let o2 = vec![1, 3, 5, 7];
    let ci1 = Ugen::IConst(IConst { value: 1 });
    let cf1 = Ugen::FConst(FConst { value: 3.3 });
    let mut ugens1: UgenList = Vec::new();
    ugens1.push(Box::new(ci1.clone()));
    ugens1.push(Box::new(cf1.clone()));
    let exu1 = extend(ugens1, 5);
    let nums = vec![13,23,38,11];
    let p1 = Ugen::Primitive(Primitive {
        name: "P1".to_string(), rate: Rate::RateDr,
        ..Primitive::default()
    });

    assert_eq!(o1, o2);
    assert_eq!(exu1.len(), 5);
    assert_eq!(max_num(nums, 21), 38);
    //assert_eq!(rate_of(p1), Rate::RateDr);
}
