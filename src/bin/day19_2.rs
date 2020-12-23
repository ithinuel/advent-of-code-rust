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

fn into_nom_parser<'a>(
    rules: &'a HashMap<usize, Rule>,
    id: usize,
) -> impl FnMut(&str) -> IResult<&str, ()> + 'a {
    let rule = &rules[&id];
    move |input| match rule {
        Either::Left(ref s) => {
            let s: &str = &s;
            map(tag(s), |_| ())(input)
        }
        Either::Right((ref subrule, None)) => {
            subrule.iter().try_fold((input, ()), |(input, _), &id| {
                into_nom_parser(rules, id)(input)
            })
        }
        Either::Right((ref subrule1, Some(ref subrule2))) => subrule1
            .iter()
            .try_fold((input, ()), |(input, _), &id| {
                into_nom_parser(rules, id)(input)
            })
            .or_else(|_| {
                subrule2.iter().try_fold((input, ()), |(input, _), &id| {
                    into_nom_parser(rules, id)(input)
                })
            }),
    }
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

    // assert the assummtion about rule 0, 8 and 11 is correct
    let magic_rules = [0, 8, 11];
    debug_assert_eq!(rules[&0], Either::Right(([8, 11].to_vec(), None)));
    debug_assert!(!rules
        .iter()
        .filter(|(id, _)| !magic_rules.contains(id))
        .any(|(_, rule)| {
            match rule {
                Either::Left(_) => false,
                Either::Right((subrule, None)) => subrule.iter().any(|id| magic_rules.contains(id)),
                Either::Right((subrule1, Some(subrule2))) => {
                    subrule1.iter().any(|id| magic_rules.contains(id))
                        || subrule2.iter().any(|id| magic_rules.contains(&id))
                }
            }
        }));

    // get parsers.
    let mut rule42 = into_nom_parser(&rules, 42);
    let mut rule31 = into_nom_parser(&rules, 31);
    let mut rule0 = |input| -> IResult<&str, ()> {
        // the only occurence of rule 8 and 11 are in rule 0 with 0: 8 11
        // so we pack together rule 0, 8, and 11
        // ungreedily try to parse 42 then (42 31)
        for n in 1.. {
            let (input, _) = count(&mut rule42, n)(input)?;
            let res = terminated(length_count(many1_count(&mut rule42), &mut rule31), eof)(input);
            if res.is_ok() {
                return Ok(("", ()));
            }
        }
        unreachable!()
    };

    // apply the rules
    let match_count = input
        .next()
        .expect("Invalid input format")
        .lines()
        .filter(|line| rule0(&line).is_ok())
        .count();
    println!("{:?}", match_count);
}
