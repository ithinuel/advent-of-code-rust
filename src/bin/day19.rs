use either::Either;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{digit1, space1},
    combinator::{eof, map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::io::Read;

// nom parser for the rules
type SubRule = Vec<usize>;
type Rule = Either<String, (SubRule, Option<SubRule>)>;

fn parse_subrule(input: &str) -> IResult<&str, SubRule> {
    separated_list1(space1, map_res(digit1, |a: &str| a.parse()))(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, Rule)> {
    terminated(
        separated_pair(
            map_res(take_till(|c| c == ':'), |v: &str| v.parse()),
            tag(": "),
            alt((
                map(
                    delimited(
                        tag("\""),
                        map(take(1usize), |v: &str| v.to_string()),
                        tag("\""),
                    ),
                    Either::Left,
                ),
                map(
                    tuple((parse_subrule, opt(preceded(tag(" | "), parse_subrule)))),
                    Either::Right,
                ),
            )),
        ),
        eof,
    )(input)
}

fn into_regex_internal(
    rules: &HashMap<usize, Rule>,
    cache: &mut HashMap<usize, String>,
    root: usize,
) -> String {
    if !cache.contains_key(&root) {
        let rule = match rules[&root] {
            Either::Left(ref s) => s.clone(),
            Either::Right((ref rule, None)) => {
                let mut new_rule = String::new();
                for id in rule {
                    new_rule += &into_regex_internal(rules, cache, *id);
                }
                new_rule
            }
            Either::Right((ref rule1, Some(ref rule2))) => {
                let mut new_rule = "((".to_string();
                for id in rule1 {
                    new_rule += &into_regex_internal(rules, cache, *id);
                }
                new_rule += ")|(";

                for id in rule2 {
                    new_rule += &into_regex_internal(rules, cache, *id);
                }
                new_rule + "))"
            }
        };
        cache.insert(root, rule);
    }
    cache[&root].clone()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from STDIN");
    let mut input = input.split("\n\n");

    let rules: HashMap<_, _> = input
        .next()
        .expect("Invalid input format")
        .lines()
        .map(|line| parse_rule(&line).expect("Invalid rule").1)
        .collect();
    println!("{:?}", rules);

    // rules to regexp
    let mut cache = HashMap::new();
    let regexp = format!("^{}$", into_regex_internal(&rules, &mut cache, 0));
    println!("{:?}", cache);
    let re = regex::Regex::new(&regexp).unwrap();

    let match_count = input
        .next()
        .expect("Invalid input format")
        .lines()
        .filter(|line| re.is_match(&line))
        .inspect(|line| println!("{}", line))
        .count();
    println!("{:?}", match_count);
}
