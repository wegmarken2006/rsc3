mod osc;
use osc::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Rate {
    RateIr = 0,
    RateKr = 1,
    RateAr = 2,
    RateDr = 3,
}

type UgenList = Vec<Box<Ugen>>;
type RateList = Vec<Rate>;
type NodeList = Vec<Node>;

#[derive(Clone, PartialEq, Debug)]
struct IConst {
    value: i32,
}
#[derive(Clone, PartialEq, Debug)]
struct FConst {
    value: f32,
}
#[derive(Clone, PartialEq, Debug)]
struct Control {
    name: String,
    index: i32,
    rate: Rate,
}

#[derive(Clone, PartialEq, Debug)]
struct Primitive {
    name: String,
    inputs: UgenList,
    outputs: RateList,
    special: i32,
    index: i32,
    rate: Rate,
}
#[derive(Clone, PartialEq, Debug)]
struct Mce {
    ugens: Vec<Box<Ugen>>,
}
#[derive(Clone, PartialEq, Debug)]
struct Mrg {
    left: Box<Ugen>,
    right: Box<Ugen>,
}
#[derive(Clone, PartialEq, Debug)]
struct Proxy {
    primitive: Primitive,
    index: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct FromPortC {
    port_nid: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct FromPortK {
    port_nid: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct FromPortU {
    port_nid: i32,
    port_idx: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct NodeC {
    id: i32,
    value: f32,
}

#[derive(Clone, PartialEq, Debug)]
struct NodeK {
    id: i32,
    name: String,
    rate: Rate,
    def: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct NodeU {
    id: i32,
    name: String,
    rate: Rate,
    inputs: UgenList,
    outputs: RateList,
    special: i32,
    ugen_id: i32,
}

#[derive(Clone, PartialEq, Debug)]
enum Node {
    NodeC(NodeC),
    NodeK(NodeK),
    NodeU(NodeU),
}

#[derive(Clone, PartialEq, Debug)]
struct Graph {
    next_id: i32,
    constants: Vec<NodeC>,
    controls: Vec<NodeK>,
    ugens: Vec<NodeU>,
}

struct Input {
    u: i32,
    p: i32,
}

struct MMap {
    cs: Vec<i32>,
    ks: Vec<i32>,
    us: Vec<i32>,
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

impl Default for NodeU {
    fn default() -> Self {
        NodeU {
            id: 0,
            name: "".to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            special: 0,
            ugen_id: 0,
            rate: Rate::RateKr,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Ugen {
    IConst(IConst),
    FConst(FConst),
    Control(Control),
    Primitive(Primitive),
    Mce(Mce),
    Mrg(Mrg),
    Proxy(Proxy),
    FromPortC(FromPortC),
    FromPortK(FromPortK),
    FromPortU(FromPortU),
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
        _ => println!("Other"),
    }
}

fn print_ugens(ugens: &UgenList) {
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

fn extend(ugens: &UgenList, new_len: i32) -> UgenList {
    let ln = ugens.len() as i32;
    let mut out: UgenList = Vec::new();
    if ln > new_len {
        out.extend_from_slice(&ugens[0..new_len as usize]);
        out
    } else {
        out.extend(ugens.clone());
        out.extend(ugens.clone());
        extend(&out.clone(), new_len)
    }
}

fn rate_id(rate: Rate) {
    rate as i32;
}

fn is_sink(ugen: &Ugen) -> bool {
    match *ugen {
        Ugen::Mce(ref mce) => {
            let mut ret = false;
            //for elem in mce.ugens {
            for elem in &mce.ugens {
                if is_sink(&*elem) {
                    ret = true;
                    break;
                }
            }
            ret
        }
        Ugen::Mrg(ref mrg) => {
            if is_sink(&*mrg.left) {
                true
            } else {
                false
            }
        }
        Ugen::Primitive(ref primitive) => {
            if primitive.inputs.len() == 0 {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn max_num(nums: Vec<i32>, start: i32) -> i32 {
    let mut max = start;
    for elem in nums {
        if elem > max {
            max = elem
        }
    }
    max
}

fn max_rate(rates: RateList, start: Rate) -> Rate {
    let mut max = start;
    for elem in rates {
        if elem as i32 > max as i32 {
            max = elem;
        }
    }
    max
}

fn rate_of(ugen: &Ugen) -> Rate {
    match ugen {
        Ugen::Control(control) => control.rate,
        Ugen::Mce(mce) => {
            let mut rates = Vec::new();
            for ugen in &mce.ugens {
                rates.push(rate_of(&*ugen));
            }
            max_rate(rates, Rate::RateKr)
        }
        Ugen::Mrg(mrg) => rate_of(&*mrg.left),
        Ugen::Primitive(primitive) => primitive.rate,
        Ugen::Proxy(proxy) => proxy.primitive.rate,
        _ => Rate::RateKr,
    }
}

fn mce_degree(ugen: &Ugen) -> i32 {
    match ugen {
        Ugen::Mce(mce) => mce.ugens.len() as i32,
        Ugen::Mrg(mrg) => mce_degree(&mrg.left),
        _ => panic!("mce_degree"),
    }
}

fn mce_extend(n: i32, ugen: &Ugen) -> UgenList {
    match ugen {
        Ugen::Mce(mce) => extend(&mce.ugens, n),
        Ugen::Mrg(mrg) => {
            let ex = mce_extend(n, &*mrg.left);
            if ex.len() <= 0 {
                panic!("mce_extend")
            }
            let mut out: UgenList = Vec::new();
            out.push(Box::new(ugen.clone()));
            out.extend_from_slice(&ex[1..n as usize]);
            out
        }
        _ => {
            let mut out: UgenList = Vec::new();
            for ind in 1..n {
                out.push(Box::new(ugen.clone()));
            }
            out.push(Box::new(ugen.clone()));
            out
        }
    }
}

fn is_mce(ugen: &Ugen) -> bool {
    match ugen {
        Ugen::Mce(mce) => true,
        _ => false,
    }
}

fn transposer<T>(list: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let len1 = list.len();
    let len2 = (list[0]).len();
    let mut out: Vec<Vec<T>> = Vec::new();

    for ind2 in 0..len2 as usize {
        let mut out1: Vec<T> = Vec::new();
        for ind1 in 0..len1 as usize {
            let in1 = &list[ind1];
            let in2 = &in1[ind2];
            out1.push((*in2).clone());
        }
        out.push(out1);
    }
    out
}

fn mce_transform(ugen: &Ugen) -> Ugen {
    match ugen {
        Ugen::Primitive(primitive) => {
            let ins: UgenList = primitive
                .clone()
                .inputs
                .into_iter()
                .filter(|x| is_mce(x))
                .collect();
            let mut degs: Vec<i32> = Vec::new();
            for elem in ins {
                degs.push(mce_degree(&elem));
            }
            let upr = max_num(degs, 0);
            let mut ext: Vec<UgenList> = Vec::new();
            for elem in primitive.clone().inputs {
                ext.push(mce_extend(upr, &*elem));
            }
            let iet = transposer(ext);
            let mut out: UgenList = Vec::new();
            let p = primitive.clone();
            let name = p.name;
            let index = p.index;
            let special = p.special;
            let rate = p.rate;
            let outputs = p.outputs;
            for elem in iet {
                let new_p = Primitive {
                    inputs: elem.clone(),
                    name: name.clone(),
                    outputs: outputs.clone(),
                    index: index,
                    special: special,
                    rate: rate,
                };
                out.push(Box::new(Ugen::Primitive(new_p)));
            }
            Ugen::Mce(Mce { ugens: out })
        }
        _ => panic!("mce_transform"),
    }
}

fn mce_expand(ugen: &Ugen) -> Ugen {
    match ugen {
        Ugen::Mce(mce) => {
            let mut lst: UgenList = Vec::new();
            for elem in &mce.ugens {
                lst.push(Box::new(mce_expand(elem)));
            }
            Ugen::Mce(Mce { ugens: lst })
        }
        Ugen::Mrg(mrg) => {
            let lst = mce_expand(&*mrg.left);
            let right = mrg.right.clone();
            Ugen::Mrg(Mrg {
                left: Box::new(lst),
                right: right,
            })
        }
        _ => {
            fn rec(ugen: &Ugen) -> bool {
                match ugen {
                    Ugen::Primitive(primitive) => {
                        let ins: UgenList = primitive
                            .clone()
                            .inputs
                            .into_iter()
                            .filter(|x| is_mce(x))
                            .collect();
                        ins.len() != 0
                    }
                    _ => false,
                }
            }
            if rec(ugen) {
                mce_expand(&mce_transform(ugen))
            } else {
                ugen.clone()
            }
        }
    }
}

fn mce_channel(n: i32, ugen: &Ugen) -> Ugen {
    match ugen {
        Ugen::Mce(mce) => *mce.ugens[n as usize].clone(),
        _ => panic!("mce_channel"),
    }
}

fn mce_channels(ugen: &Ugen) -> UgenList {
    match ugen {
        Ugen::Mce(mce) => mce.ugens.clone(),
        Ugen::Mrg(mrg) => {
            let lst = mce_channels(&*mrg.left);
            if lst.len() <= 1 {
                panic!("mce_channels");
            }
            let mrg1 = Ugen::Mrg(Mrg {
                left: lst[0].clone(),
                right: mrg.right.clone(),
            });
            let mut out: UgenList = vec![Box::new(mrg1)];
            out.extend_from_slice(&lst[1..]);
            out
        }
        _ => {
            let out: UgenList = vec![Box::new(ugen.clone())];
            out
        }
    }
}

fn proxify(ugen: &Ugen) -> Ugen {
    match ugen {
        Ugen::Mce(mce) => {
            let mut lst: UgenList = Vec::new();
            for elem in &mce.ugens {
                lst.push(Box::new(proxify(elem)));
            }
            Ugen::Mce(Mce { ugens: lst })
        }
        Ugen::Mrg(mrg) => {
            let prx = proxify(&mrg.left);
            Ugen::Mrg(Mrg {
                left: Box::new(prx),
                right: mrg.right.clone(),
            })
        }
        Ugen::Primitive(primitive) => {
            let ln = primitive.inputs.len();
            if ln < 2 {
                return ugen.clone();
            }
            let lst1 = iota(ln as i32, 0, 1);
            let mut lst2: UgenList = Vec::new();
            for index in lst1 {
                let proxy = Ugen::Proxy(Proxy {
                    index: index,
                    primitive: primitive.clone(),
                });
                lst2.push(Box::new(proxy));
            }
            Ugen::Mce(Mce { ugens: lst2 })
        }
        _ => panic!("proxify"),
    }
}

fn mk_ugen(
    rate: Rate,
    name: String,
    inputs: UgenList,
    outputs: RateList,
    ind: i32,
    sp: i32,
) -> Ugen {
    let spr1 = Primitive {
        name: name,
        inputs: inputs,
        outputs: outputs,
        special: sp,
        index: ind,
        rate: rate,
    };
    let pr1 = Ugen::Primitive(spr1);
    proxify(&pr1)
}

fn mode_c_value(node: &Node) -> f32 {
    match node {
        Node::NodeC(nodec) => nodec.value,
        _ => panic!("node_c_value"),
    }
}
fn node_k_default(node: &Node) -> i32 {
    match node {
        Node::NodeK(nodek) => nodek.def,
        _ => panic!("node_k_default"),
    }
}

fn mk_map(gr1: &Graph) -> MMap {
    let mut cs = Vec::new();
    let mut ks = Vec::new();
    let mut us = Vec::new();
    let gr = gr1.clone();
    for elem in gr.constants {
        cs.push(elem.id);
    }
    for elem in gr.controls {
        ks.push(elem.id);
    }
    for elem in gr.ugens {
        us.push(elem.id);
    }
    MMap {
        cs: cs,
        ks: ks,
        us: us,
    }
}

fn fetch(val: i32, lst: Vec<i32>) -> i32 {
    for (ind, elem) in lst.into_iter().enumerate() {
        if elem == val {
            return ind as i32;
        }
    }
    return -1;
}

fn as_from_port(node: &Node) -> Ugen {
    match node {
        Node::NodeC(nodec) => Ugen::FromPortC(FromPortC { port_nid: nodec.id }),
        Node::NodeK(nodek) => Ugen::FromPortK(FromPortK { port_nid: nodek.id }),
        Node::NodeU(nodeu) => Ugen::FromPortU(FromPortU {
            port_nid: nodeu.id,
            port_idx: 0,
        }),
    }
}

fn find_c_p(val: f32, nodec: &NodeC) -> bool {
    val == nodec.value
}

fn push_c(val: f32, gr: &Graph) -> (Node, Graph) {
    let node = NodeC {
        id: gr.next_id + 1,
        value: val,
    };
    let mut consts = vec![node.clone()];
    consts.extend(gr.constants.clone());
    let gr1 = Graph {
        next_id: gr.next_id + 1,
        constants: consts,
        controls: gr.controls.clone(),
        ugens: gr.ugens.clone(),
    };
    (Node::NodeC(node), gr1)
}

fn mk_node_c(ugen: &Ugen, gr: &Graph) -> (Node, Graph) {
    let val: f32 = match ugen {
        Ugen::IConst(iconst) => iconst.value as f32,
        Ugen::FConst(fconst) => fconst.value,
        _ => panic!("mk_node_c"),
    };
    for nodec in &gr.constants {
        if find_c_p(val, &nodec) {
            return (Node::NodeC(nodec.clone()), gr.clone());
        }
    }
    return push_c(val, &gr.clone());
}

fn find_k_p(name: &String, nodek: &NodeK) -> bool {
    *name == nodek.name
}

fn push_k(ctrl: &Control, gr: &Graph) -> (Node, Graph) {
    let node = NodeK {
        id: gr.next_id + 1,
        name: ctrl.name.clone(),
        def: ctrl.index,
        rate: ctrl.rate,
    };
    let mut contrs = vec![node.clone()];
    contrs.extend(gr.controls.clone());
    let gr1 = Graph {
        next_id: gr.next_id + 1,
        constants: gr.constants.clone(),
        controls: contrs,
        ugens: gr.ugens.clone(),
    };
    (Node::NodeK(node), gr1)
}

fn mk_node_k(ugen: &Ugen, gr: &Graph) -> (Node, Graph) {
    let control = match ugen {
        Ugen::Control(contr) => contr,
        _ => panic!("mk_node_k"),
    };
    let name = &control.name;
    for nodek in &gr.controls {
        if find_k_p(name, &nodek) {
            return (Node::NodeK(nodek.clone()), gr.clone());
        }
    }
    return push_k(&control, &gr.clone());
}

fn find_u_p(rate: Rate, name: &String, id: i32, node: &NodeU) -> bool {
    if node.rate == rate && node.name == *name && node.id == id {
        return true;
    }
    return false;
}

fn push_u(primitive: &Primitive, gr: &Graph) -> (Node, Graph) {
    let node = NodeU {
        id: gr.next_id + 1,
        name: primitive.name.clone(),
        rate: primitive.rate,
        inputs: primitive.inputs.clone(),
        outputs: primitive.outputs.clone(),
        special: primitive.special,
        ugen_id: primitive.index,
    };
    let mut ugens = vec![node.clone()];
    ugens.extend(gr.ugens.clone());
    let gr1 = Graph {
        next_id: gr.next_id + 1,
        constants: gr.constants.clone(),
        controls: gr.controls.clone(),
        ugens: ugens,
    };
    (Node::NodeU(node), gr1)
}

fn acc(mut ll: UgenList, mut nn: NodeList, gr: &Graph) -> (NodeList, Graph) {
    if ll.len() == 0 {
        nn.clone().reverse();
        return (nn, gr.clone());
    } else {
        let (ng1, ng2) = mk_node(&ll[0], gr);
        nn.insert(0, ng1);
        ll.drain(0..1);
        return acc(ll, nn, &ng2);
    }
}

fn mk_node_u(ugen: &Ugen, gr: &Graph) -> (Node, Graph) {
    let primitive = match ugen {
        Ugen::Primitive(primitive) => primitive,
        _ => panic!("mk_node_u"),
    };
    let (ng1, gnew) = acc(primitive.inputs.clone(), Vec::new(), gr);
    let mut inputs2 = Vec::new();
    for nd in ng1 {
        inputs2.push(Box::new(as_from_port(&nd)));
    }
    let name = primitive.name.clone();
    let rate = primitive.rate;
    let index = primitive.index;
    for nd2 in &gnew.ugens {
        if find_u_p(rate, &name, index, nd2) {
            return (Node::NodeU(nd2.clone()), gnew.clone());
        }
    }
    let pr = Primitive {
        name: name,
        inputs: inputs2,
        outputs: primitive.outputs.clone(),
        special: primitive.special,
        index: index,
        rate: rate,
    };
    return push_u(&pr, &gnew);
}

fn mk_node(ugen: &Ugen, gr: &Graph) -> (Node, Graph) {
    match ugen {
        Ugen::IConst(iconst) => mk_node_c(ugen, gr),
        Ugen::FConst(fconst) => mk_node_c(ugen, gr),
        Ugen::Control(control) => mk_node_k(ugen, gr),
        Ugen::Primitive(primitive) => mk_node_c(ugen, gr),
        Ugen::Mrg(mrg) => {
            let (_, gg) = mk_node(&*mrg.right, gr);
            mk_node(&*mrg.left, &gg)
        }
        _ => panic!("mk_node"),
    }
}

fn implicit(num: i32) -> NodeU {
    let mut rates = Vec::new();
    for ind in 1..(num + 1) {
        rates.push(Rate::RateKr);
    }
    let node = NodeU {
        id: -1,
        name: "Control".to_string(),
        rate: Rate::RateKr,
        inputs: Vec::new(),
        outputs: rates,
        special: 0,
        ugen_id: 0,
    };
    node
}

fn mrg_n(lst: &UgenList) -> Ugen {
    if lst.len() == 1 {
        return *lst[0].clone();
    } else if lst.len() == 2 {
        return Ugen::Mrg(Mrg {
            left: lst[0].clone(),
            right: lst[1].clone(),
        });
    }
    let mut newlst = Vec::new();
    newlst.extend(lst.clone());
    return Ugen::Mrg(Mrg {
        left: lst[0].clone(),
        right: Box::new(mrg_n(&newlst)),
    });
}

fn prepare_root(ugen: &Ugen) -> Ugen {
    match ugen {
        Ugen::Mce(mce) => mrg_n(&mce.ugens),
        Ugen::Mrg(mrg) => {
            let m1 = Mrg {
                left: Box::new(prepare_root(&*mrg.left)),
                right: Box::new(prepare_root(&*mrg.right)),
            };
            Ugen::Mrg(m1)
        }
        _ => ugen.clone(),
    }
}

fn empty_graph() -> Graph {
    Graph{
        next_id: 0,
        constants: Vec::new(),
        controls: Vec::new(),
        ugens: Vec::new(),        
    }
}

fn synth(ugen: &Ugen) -> Graph {
	let root = prepare_root(ugen);
	let (_, gr) = mk_node(&root, &empty_graph());
	let cs = gr.constants.clone();
	let ks = gr.controls.clone();
	let us = gr.ugens.clone();
	//reverse us
	let mut us1 = Vec::new();
    us1.extend(us);
	if ks.len() != 0 {
		let node = implicit(ks.len() as i32);
        us1.insert(0, node);
	}
	let grout = Graph{next_id: -1, constants: cs, controls: ks, ugens: us1};
	grout
}

////utilities
fn iconst(val: i32) -> Box<Ugen> {
    Box::new(Ugen::IConst(IConst { value: val }))
}

fn mk_ugenlist(vargs: &[&Ugen]) -> UgenList {
    let mut out: UgenList = Vec::new();
    for elem in vargs {
        out.push(Box::new((*elem).clone()));
    }
    out
}

fn mk_mce(ugens: UgenList) -> Ugen {
    Ugen::Mce(Mce { ugens: ugens })
}

fn get_primitive(ugen: &Ugen) -> Primitive {
    match ugen {
        Ugen::Primitive(primitive) => primitive.clone(),
        _ => panic!("get_primitive"),
    }
}

fn get_node_c(node: &Node) -> NodeC {
    match node {
        Node::NodeC(nodec) => nodec.clone(),
        _ => panic!("get_node_c"),
    }
}
fn get_node_k(node: &Node) -> NodeK {
    match node {
        Node::NodeK(nodek) => nodek.clone(),
        _ => panic!("get_node_k"),
    }
}
fn get_node_u(node: &Node) -> NodeU {
    match node {
        Node::NodeU(nodeu) => nodeu.clone(),
        _ => panic!("get_node_u"),
    }
}

//////
fn main() {
    println!("start");
    let o1 = iota(4, 1, 2);
    let o2 = vec![1, 3, 5, 7];
    let ci1 = Ugen::IConst(IConst { value: 1 });
    let cf1 = Ugen::FConst(FConst { value: 3.3 });
    let mut ugens1: UgenList = Vec::new();
    ugens1.push(Box::new(ci1.clone()));
    ugens1.push(Box::new(cf1.clone()));
    let exu1 = extend(&ugens1, 5);
    let nums = vec![13, 23, 38, 11];
    let p1 = Ugen::Primitive(Primitive {
        name: "P1".to_string(),
        inputs: ugens1.clone(),
        outputs: vec![Rate::RateKr, Rate::RateIr],
        ..Primitive::default()
    });
    let pp1 = get_primitive(&p1);
    let p2 = Ugen::Primitive(Primitive {
        name: "P2".to_string(),
        rate: Rate::RateAr,
        ..Primitive::default()
    });

    let mc1 = Ugen::Mce(Mce {
        ugens: vec![Box::new(p1.clone()), Box::new(p2.clone())],
    });
    let mc2 = mk_mce(mk_ugenlist(&[&p1, &p2]));
    let mc3 = mk_mce(mk_ugenlist(&[&p1, &p2, &mc1]));
    let mg1 = Ugen::Mrg(Mrg {
        left: Box::new(mc1.clone()),
        right: Box::new(p1.clone()),
    });

    let p3 = Ugen::Primitive(Primitive {
        name: "P3".to_string(),
        rate: Rate::RateKr,
        inputs: mk_ugenlist(&[&mc1, &mc3]),
        outputs: vec![Rate::RateIr],
        ..Primitive::default()
    });
    let mc10 = mce_transform(&p3);
    let mc101 = match mc10 {
        Ugen::Mce(mce) => mce,
        _ => panic!("mce_transform test"),
    };
    let pp3 = &mc101.ugens[2];
    let pp31 = match *(*pp3).clone() {
        Ugen::Primitive(prim) => prim,
        _ => panic!("mce_transform test 2"),
    };
    let mg3 = Ugen::Mrg(Mrg {
        left: Box::new(mc1.clone()),
        right: Box::new(p2.clone()),
    });
    let ndk1 = Node::NodeK(NodeK {
        name: "ndk1".to_string(),
        def: 5,
        id: 30,
        rate: Rate::RateKr,
    });
    let ndk2 = Node::NodeK(NodeK {
        name: "ndk2".to_string(),
        def: 5,
        id: 31,
        rate: Rate::RateKr,
    });
    let ndc1 = Node::NodeC(NodeC {
        id: 20,
        value: 320 as f32,
    });
    let ndc2 = Node::NodeC(NodeC {
        id: 21,
        value: 321 as f32,
    });
    let ndu1 = Node::NodeU(NodeU {
        id: 40,
        name: "ndu1".to_string(),
        rate: Rate::RateDr,
        special: 11,
        ugen_id: 2,
        ..NodeU::default()
    });
    let ndu2 = Node::NodeU(NodeU {
        id: 41,
        name: "ndu2".to_string(),
        ..NodeU::default()
    });
    let gr1 = Graph {
        next_id: 11,
        constants: vec![get_node_c(&ndc1), get_node_c(&ndc2)],
        controls: vec![get_node_k(&ndk1), get_node_k(&ndk2)],
        ugens: vec![get_node_u(&ndu1), get_node_u(&ndu2)],
    };
    let (nn10, _) = mk_node_c(&iconst(320), &gr1);
    let nnc10 = get_node_c(&nn10);
    let ck1 = Ugen::Control(Control {
        name: "ndk1".to_string(),
        rate: Rate::RateKr,
        index: 3,
    });
    let (nn11, _) = mk_node_k(&ck1, &gr1);

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
    let exu1 = extend(&ugens1, 5);
    let nums = vec![13, 23, 38, 11];
    let p1 = Ugen::Primitive(Primitive {
        name: "P1".to_string(),
        inputs: ugens1.clone(),
        outputs: vec![Rate::RateKr, Rate::RateIr],
        ..Primitive::default()
    });
    let p2 = Ugen::Primitive(Primitive {
        name: "P2".to_string(),
        rate: Rate::RateAr,
        ..Primitive::default()
    });

    let mc1 = Ugen::Mce(Mce {
        ugens: vec![Box::new(p1.clone()), Box::new(p2.clone())],
    });
    let mc2 = mk_mce(mk_ugenlist(&[&p1, &p2]));
    let mc3 = mk_mce(mk_ugenlist(&[&p1, &p2, &mc1]));
    let mg1 = Ugen::Mrg(Mrg {
        left: Box::new(mc1.clone()),
        right: Box::new(p1.clone()),
    });
    let ex1 = mce_extend(3, &mg1);
    let ic1 = vec![
        vec![iconst(1), iconst(2)],
        vec![iconst(3), iconst(4)],
        vec![iconst(5), iconst(6)],
    ];
    let l2 = transposer(ic1.clone());

    let p3 = Ugen::Primitive(Primitive {
        name: "P3".to_string(),
        rate: Rate::RateKr,
        inputs: mk_ugenlist(&[&mc1, &mc3]),
        outputs: vec![Rate::RateIr],
        ..Primitive::default()
    });
    let mc10 = mce_transform(&p3);
    let mc101 = match mc10 {
        Ugen::Mce(mce) => mce,
        _ => panic!("mce_transform test"),
    };
    let pp3 = &mc101.ugens[2];
    let pp31 = match *(*pp3).clone() {
        Ugen::Primitive(prim) => prim,
        _ => panic!("mce_transform test 2"),
    };
    let mg3 = Ugen::Mrg(Mrg {
        left: Box::new(mc1.clone()),
        right: Box::new(p2.clone()),
    });
    let l22 = mce_channels(&mg3);
    let el10 = &(*l22[0]);
    let el11 = &(*l22[1]);
    let el10t = match el10 {
        Ugen::Mrg(mrg) => mrg,
        _ => panic!("mce_channel test"),
    };
    let el11t = match el11 {
        Ugen::Primitive(primitive) => primitive,
        _ => panic!("mce_channel test 2"),
    };
    let prx1 = proxify(&mc2);
    let l23 = match prx1 {
        Ugen::Mce(mce) => mce,
        _ => panic!("proxify test"),
    };
    let el12 = l23.ugens[0].clone();
    let el13 = l23.ugens[1].clone();
    let el12t = match *el12 {
        Ugen::Mce(mce) => mce,
        _ => panic!("proxify test 1"),
    };
    let el13t = match *el13 {
        Ugen::Primitive(primitive) => primitive,
        _ => panic!("proxify test 2"),
    };
    let b1 = encode_i16(125);
    let ndk1 = Node::NodeK(NodeK {
        name: "ndk1".to_string(),
        def: 5,
        id: 30,
        rate: Rate::RateKr,
    });
    let ndk2 = Node::NodeK(NodeK {
        name: "ndk2".to_string(),
        def: 5,
        id: 31,
        rate: Rate::RateKr,
    });
    let ndc1 = Node::NodeC(NodeC {
        id: 20,
        value: 320 as f32,
    });
    let ndc2 = Node::NodeC(NodeC {
        id: 21,
        value: 321 as f32,
    });
    let ndu1 = Node::NodeU(NodeU {
        id: 40,
        name: "ndu1".to_string(),
        rate: Rate::RateDr,
        special: 11,
        ugen_id: 2,
        ..NodeU::default()
    });
    let ndu2 = Node::NodeU(NodeU {
        id: 41,
        name: "ndu2".to_string(),
        ..NodeU::default()
    });
    let gr1 = Graph {
        next_id: 11,
        constants: vec![get_node_c(&ndc1), get_node_c(&ndc2)],
        controls: vec![get_node_k(&ndk1), get_node_k(&ndk2)],
        ugens: vec![get_node_u(&ndu1), get_node_u(&ndu2)],
    };
    let mm1 = mk_map(&gr1);
    let lc1 = mm1.cs;
    let lk1 = mm1.ks;
    let lu1 = mm1.us;
    let (nn10, _) = mk_node_c(&iconst(320), &gr1);
    let nnc10 = get_node_c(&nn10);
    let ck1 = Ugen::Control(Control {
        name: "ndk1".to_string(),
        rate: Rate::RateKr,
        index: 3,
    });
    let (nn11, _) = mk_node_k(&ck1, &gr1);
    let nnk11 = get_node_k(&nn11);

    assert_eq!(o1, o2);
    assert_eq!(exu1.len(), 5);
    assert_eq!(max_num(nums, 21), 38);
    assert_eq!(is_sink(&p2), true);
    assert_eq!(rate_of(&p2), Rate::RateAr);
    assert_eq!(mce_degree(&mc1), 2);
    assert_eq!(ex1.len(), 3);
    assert_eq!(l2.len(), 2);
    assert_eq!(pp31.name, "P3".to_string());
    assert_eq!(decode_i16(b1), 125);
    assert_eq!(lc1[0], 20);
    assert_eq!(lk1[1], 31);
    assert_eq!(lu1[0], 40);
    assert_eq!(find_c_p(320 as f32, &get_node_c(&ndc1)), true);
    assert_eq!(nnc10.id, 20);
    assert_eq!(nnk11.id, 30);
}
