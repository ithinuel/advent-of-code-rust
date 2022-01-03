extern crate regex;

use std::cmp::max;
use std::collections::BTreeMap;
use std::io::stdin;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut prgm = String::new();
    let _ = stdin().read_to_string(&mut prgm);

    let re = Regex::new(r"(?P<output>\w+) (?P<action>inc|dec) (?P<op>-?\d+) if (?P<cond>\w+) (?P<test><|<=|>|>=|==|!=) (?P<val>-?\d+)").unwrap();

    let mut regs: BTreeMap<String, i32> = BTreeMap::new();
    let mut max_value = 0;

    for cap in re.captures_iter(&prgm) {
        let test = {
            let cond = *regs.entry(cap["cond"].to_string()).or_insert(0);
            let val = cap["val"].parse().unwrap();
            match &cap["test"] {
                "<" => cond < val,
                "<=" => cond <= val,
                ">" => cond > val,
                ">=" => cond >= val,
                "==" => cond == val,
                "!=" => cond != val,
                _ => panic!("Invalid operand"),
            }
        };

        if test {
            let output = regs.entry(cap["output"].to_string()).or_insert(0);
            let val: i32 = cap["op"].parse().unwrap();
            match &cap["action"] {
                "inc" => *output += val,
                "dec" => *output -= val,
                _ => panic!("Invalid instruction"),
            }

            max_value = max(max_value, *output);
        }
    }
    println!("{:?}", regs);
    println!("{}", regs.values().fold(0, |m, v| max(m, *v)));
    println!("{}", max_value);
}
