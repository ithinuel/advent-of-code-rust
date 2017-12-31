use std::io::stdin;
use std::io::BufRead;
use std::collections::BTreeMap;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;

#[derive(Debug, Clone)]
enum Arg {
    Val(i64),
    Reg(String),
}

impl Arg {
    fn new(val: &str) -> Arg {
        val.parse()
            .and_then(|v| Ok(Arg::Val(v)))
            .unwrap_or_else(|_| Arg::Reg(val.to_string()))
    }
    fn get(&self, regs: &BTreeMap<String, i64>) -> i64 {
        *match self {
            &Arg::Val(ref v) => v,
            &Arg::Reg(ref name) => {
                regs.get(name).unwrap_or(&0)
            }
        }
    }
}


#[derive(Debug, Clone)]
enum Inst {
    Snd(Arg),
    Set(String, Arg),
    Add(String, Arg),
    Mul(String, Arg),
    Mod(String, Arg),
    Rcv(String),
    Jgz(Arg, Arg), 
}

fn run(prgm: &Vec<Inst>, id: i64, receiver: Receiver<i64>, sender: Sender<i64>) -> usize {
    let mut regs: BTreeMap<String, i64> = BTreeMap::new();
    regs.insert("p".to_string(), id);
    let mut pc = 0;
    let mut send_count = 0;
    while pc < prgm.len() {
        //if id == 1 { println!("{}: {:?} {} {:?}", id, regs, pc, prgm[pc]); }
        pc = ((pc as i64) + match &prgm[pc] {
            &Inst::Snd(ref op) => {
                let snd = op.get(&regs);
                send_count += 1;
                sender.send(snd).unwrap();
                //println!("{}: {:?} {} {:?}", id, regs, pc, prgm[pc]);
                1
            }
            &Inst::Set(ref reg, ref op) => {
                let v = op.get(&regs);
                regs.insert(reg.clone(), v);
                1
            },
            &Inst::Add(ref reg, ref op) => {
                let r = *regs.get(reg).unwrap_or(&0) + op.get(&regs);
                regs.insert(reg.clone(), r);
                1
            }
            &Inst::Mul(ref reg, ref op) => {
                let r = *regs.get(reg).unwrap_or(&0) * op.get(&regs);
                regs.insert(reg.clone(), r);
                1
            }
            &Inst::Mod(ref reg, ref op) => {
                let r = *regs.get(reg).unwrap_or(&0) % op.get(&regs);
                regs.insert(reg.clone(), r);
                1
            }
            &Inst::Rcv(ref reg) => {
                // indirect/unsafe way to detect deadlocks.
                // the other program may just be busy and not blocked on a rcv.
                let v = receiver.recv_timeout(Duration::new(2, 0));
                match v {
                    Err(_) => break,
                    Ok(v) => regs.insert(reg.clone(), v)
                };                
                1
            }
            &Inst::Jgz(ref op1, ref op2) => {
                if op1.get(&regs) > 0 {
                    op2.get(&regs)
                } else {
                    1
                }
            }
        }) as usize;
    }
    send_count
}

fn main() {
    let input = stdin();
    let prgm: Vec<Inst> = input.lock()
        .lines()
        .map(|l| {
            let s = l.unwrap();
            let v: Vec<&str> = s.split(" ").collect();
            match v[0] {
                "snd" => Inst::Snd(Arg::new(v[1])),
                "set" => Inst::Set(v[1].to_string(), Arg::new(v[2])),
                "add" => Inst::Add(v[1].to_string(), Arg::new(v[2])),
                "mul" => Inst::Mul(v[1].to_string(), Arg::new(v[2])),
                "mod" => Inst::Mod(v[1].to_string(), Arg::new(v[2])),
                "rcv" => Inst::Rcv(v[1].to_string()),
                "jgz" => Inst::Jgz(Arg::new(v[1]), Arg::new(v[2])),
                _ => panic!("woops")
            }
        }).collect();
    let (atx, arx) = channel();
    let (btx, brx) = channel();
    println!("{:?}", prgm);

    let aprgm = prgm.clone();
    thread::spawn(move || run(&aprgm, 0, arx, btx));
    let b = thread::spawn(move || run(&prgm, 1, brx, atx));
    println!("{}", b.join().unwrap());
}
