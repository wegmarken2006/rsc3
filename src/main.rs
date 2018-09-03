

#[derive(Clone, Debug)]
enum Rate {
    RateIr=0,
    RateKr=1,
    RateAr=2,
    RateDr=3
}

//type UgenList<'a> = &'a[Box<Ugen>];
type UgenList = Vec<Box<Ugen>>;
type RateList = Vec<Rate>;

#[derive(Clone, Debug)]
struct	IConst{value: i32}
#[derive(Clone, Debug)]
struct	FConst{value: f32}
#[derive(Clone, Debug)]
struct  Control{name: String, index: i32, rate: Rate}
#[derive(Clone, Debug)]
struct  Primitive{name: String, inputs: UgenList, outputs: RateList, special: i32, 
              index: i32, rate: Rate}
#[derive(Clone, Debug)]
struct  Mce{ugens: Vec<Box<Ugen>>}
#[derive(Clone, Debug)]
struct  Mrg{left: Box<Ugen>, right: Box<Ugen>}
#[derive(Clone, Debug)]
struct  Proxy{primitive: Primitive, index: i32}

impl Default for Primitive {
    fn default() -> Self {
        Primitive{name: "".to_string(), inputs: Vec::new(), outputs: Vec::new(), special: 0, 
              index: 0, rate: Rate::RateKr}
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
    Proxy(Proxy)
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
        Ugen::IConst(IConst) => {println!("I Value: {}", IConst.value)}
        Ugen::FConst(FConst) => {println!("F Value: {}", FConst.value)}
        Ugen::Control(Control) => {println!("K Name: {}", Control.name)}
        Ugen::Mce(Mce) => {println!("MC Ulen: {}", Mce.ugens.len())}
        Ugen::Mrg(Mrg) => {println!("MG ")},
        Ugen::Primitive(Primitive) => {println!("P Name: {} IL:{}, OL:{}", 
            Primitive.name, Primitive.inputs.len(), Primitive.outputs.len())},
        Ugen::Proxy(Proxy) => {println!("Px Name:{}", Proxy.primitive.name)}
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
    }
    else {
        let mut out = Vec::new();
        out.push(init);
        out.extend(iota(n-1, init+step, step));
        out
    }
}

fn extend(ugens: UgenList, new_len: i32) -> UgenList {
    let ln = ugens.len() as i32;
    let mut out: UgenList = Vec::new();
    if ln > new_len {        
        out.extend_from_slice(&ugens[0 .. new_len as usize]);
        out
    }
    else {
        out.extend(ugens.clone());
        out.extend(ugens);
        extend(out.clone(), new_len)
    }
}

fn rate_id(rate: Rate) {
    rate as i32;
}

//////
fn main() {
    println!("start");
    let o1 = iota(4, 1, 2);
    let o2 = vec![1, 3, 5, 7];
    //let ci1 = Ugen::IConst{value: 1};
    let ci1 = Ugen::IConst(IConst{value: 1});
    let cf1 = Ugen::FConst(FConst{value: 3.3});
    let mut ugens1 : UgenList = Vec::new();
    ugens1.push(Box::new(ci1.clone()));
    ugens1.push(Box::new(cf1.clone()));
    let p1 = Ugen::Primitive(Primitive{name: "P1".to_string(), .. Primitive::default()});
    print_ugen(&p1);
    println!("end");  
    
}

#[test]
fn test1() {
    let o1 = iota(4, 1, 2);
    let o2 = vec![1, 3, 5, 7];
    let ci1 = Ugen::IConst{value: 1};
    let cf1 = Ugen::FConst{value: 3.3};
    let mut ugens1 : UgenList = Vec::new();
    ugens1.push(Box::new(ci1.clone()));
    ugens1.push(Box::new(cf1.clone()));
    let exu1 = extend(ugens1, 5);
    assert_eq!(o1, o2);
    assert_eq!(exu1.len(), 5);
}