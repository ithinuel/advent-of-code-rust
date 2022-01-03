use std::collections::BTreeMap;
use std::io::stdin;
use std::io::BufRead;

#[derive(Debug)]
enum Arg {
    Val(i64),
    Reg(String),
}

impl Arg {
    fn new(val: &str) -> Arg {
        val.parse()
            .map(Arg::Val)
            .unwrap_or_else(|_| Arg::Reg(val.to_string()))
    }
    fn get(&self, regs: &BTreeMap<String, i64>) -> i64 {
        *match self {
            Arg::Val(ref v) => v,
            Arg::Reg(ref name) => regs.get(name).unwrap_or(&0),
        }
    }
}

#[derive(Debug)]
enum Inst {
    Snd(Arg),
    Set(String, Arg),
    Add(String, Arg),
    Mul(String, Arg),
    Mod(String, Arg),
    Rcv(String),
    Jgz(String, Arg),
}

fn main() {
    let input = stdin();
    let prgm: Vec<Inst> = input
        .lock()
        .lines()
        .map(|l| {
            let s = l.unwrap();
            let v: Vec<&str> = s.split(' ').collect();
            match v[0] {
                "snd" => Inst::Snd(Arg::new(v[1])),
                "set" => Inst::Set(v[1].to_string(), Arg::new(v[2])),
                "add" => Inst::Add(v[1].to_string(), Arg::new(v[2])),
                "mul" => Inst::Mul(v[1].to_string(), Arg::new(v[2])),
                "mod" => Inst::Mod(v[1].to_string(), Arg::new(v[2])),
                "rcv" => Inst::Rcv(v[1].to_string()),
                "jgz" => Inst::Jgz(v[1].to_string(), Arg::new(v[2])),
                _ => panic!("woops"),
            }
        })
        .collect();
    println!("{:?}", prgm);

    let mut regs: BTreeMap<String, i64> = BTreeMap::new();
    let mut pc = 0;
    let mut snd = 0;
    loop {
        if pc >= prgm.len() {
            break;
        }

        println!("{:?}", regs);
        println!("{} {:?}", pc, prgm[pc]);
        pc = ((pc as i64)
            + match prgm[pc] {
                Inst::Snd(ref op) => {
                    snd = op.get(&regs);
                    1
                }
                Inst::Set(ref reg, ref op) => {
                    let v = op.get(&regs);
                    regs.insert(reg.clone(), v);
                    1
                }
                Inst::Add(ref reg, ref op) => {
                    let r = *regs.get(reg).unwrap_or(&0) + op.get(&regs);
                    regs.insert(reg.clone(), r);
                    1
                }
                Inst::Mul(ref reg, ref op) => {
                    let r = *regs.get(reg).unwrap_or(&0) * op.get(&regs);
                    regs.insert(reg.clone(), r);
                    1
                }
                Inst::Mod(ref reg, ref op) => {
                    let r = *regs.get(reg).unwrap_or(&0) % op.get(&regs);
                    regs.insert(reg.clone(), r);
                    1
                }
                Inst::Rcv(ref reg) => {
                    if *regs.get(reg).unwrap_or(&0) != 0 {
                        regs.insert(reg.clone(), snd);
                        println!("{}", snd);
                        break;
                    }
                    1
                }
                Inst::Jgz(ref reg, ref op) => {
                    if *regs.get(reg).unwrap_or(&0) > 0 {
                        op.get(&regs)
                    } else {
                        1
                    }
                }
            }) as usize;
    }
}
