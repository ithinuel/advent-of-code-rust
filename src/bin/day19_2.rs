use either::Either;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{digit1, space1},
    combinator::{eof, map, map_res, opt},
    multi::{count, length_count, many1_count, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

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

fn into_nom_parser<'a>(
    rules: &'a HashMap<usize, Rule>,
) -> Rc<RefCell<HashMap<usize, RefCell<Box<dyn FnMut(&str) -> IResult<&str, ()> + 'a>>>>> {
    let rc_cache: Rc<RefCell<HashMap<_, RefCell<Box<dyn FnMut(&str) -> IResult<&str, ()> + 'a>>>>> =
        Rc::new(RefCell::new(HashMap::new()));
    rules.iter().for_each(|(&id, rule)| {
        // some parser are too greedy… need to find an alternative
        let rule: Box<dyn FnMut(&str) -> IResult<&str, ()>> = if [0, 8, 11].contains(&id) {
            println!("Skiping {}", id);
            return;
        } else {
            match rule {
                Either::Left(ref s) => {
                    let s: &str = s;
                    Box::new(move |input| map(tag(s), |_| ())(input))
                }
                Either::Right((ref subrule, None)) => {
                    let rc_cache = rc_cache.clone();
                    Box::new(move |input| {
                        subrule.iter().try_fold((input, ()), |(input, _), &id| {
                            rc_cache.borrow()[&id].borrow_mut()(input)
                        })
                    })
                }
                Either::Right((ref subrule1, Some(ref subrule2))) => {
                    let rc_cache = rc_cache.clone();
                    Box::new(move |input| {
                        subrule1
                            .iter()
                            .try_fold((input, ()), |(input, _), &id| {
                                rc_cache.borrow()[&id].borrow_mut()(input)
                            })
                            .or_else(|_| {
                                subrule2.iter().try_fold((input, ()), |(input, _), &id| {
                                    rc_cache.borrow()[&id].borrow_mut()(input)
                                })
                            })
                    })
                }
            }
        };
        rc_cache.borrow_mut().insert(id, RefCell::new(rule));
    });
    rc_cache
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

    //let new_rule_8 = || Either::Right(([42].to_vec(), Some([42, 8].to_vec()))); // \g<42>+
    //let new_rule_11 = || Either::Right(([42, 31].to_vec(), Some([42, 11, 31].to_vec()))); // not regex

    //rules
    //.entry(8)
    //.and_modify(|rule| *rule = new_rule_8())
    //.or_insert_with(new_rule_8);
    //rules
    //.entry(11)
    //.and_modify(|rule| *rule = new_rule_11())
    //.or_insert_with(new_rule_11);
    let cache = into_nom_parser(&rules);

    let rule0: Box<dyn FnMut(&str) -> IResult<&str, ()>> = {
        let cache = cache.clone();
        Box::new(move |input| {
            // the only occurence of rule 8 and 11 are in rule 0 with 0: 8 11
            // so we pack together rule 0, 8, and 11

            let cache = cache.borrow();
            let mut rule42 = cache[&42].borrow_mut();
            let mut rule31 = cache[&31].borrow_mut();

            // ungreedily try to parse 42 then (42 31)
            for n in 1.. {
                let (input, _) = count(&mut *rule42, n)(input)?;
                let res =
                    terminated(length_count(many1_count(&mut *rule42), &mut *rule31), eof)(input);
                if res.is_ok() {
                    return Ok(("", ()));
                }
            }
            unreachable!()
        })
    };
    cache.borrow_mut().insert(0, RefCell::new(rule0));

    println!("{:?}", cache.borrow().keys().collect::<Vec<_>>());

    let match_count = input
        .next()
        .expect("Invalid input format")
        .lines()
        .enumerate()
        .filter(|(i, line)| {
            print!("{}-{}:", i, line);
            let cache = cache.borrow();
            let mut rule0 = cache[&0].borrow_mut();
            let res = rule0(&line).is_ok();
            println!("{}", res);
            res
        })
        .count();
    println!("{:?}", match_count);
}
