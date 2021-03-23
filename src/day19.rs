use aoc_runner_derive::{aoc, aoc_generator};
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

#[aoc_generator(day19)]
fn day19(input: &str) -> anyhow::Result<(String, HashMap<usize, Rule>)> {
    let mut input = input.split("\n\n");
    let invalid_format = || anyhow::anyhow!("Invalid input format");

    let rules = input
        .next()
        .ok_or_else(invalid_format)?
        .lines()
        .map(|line| parse_rule(&line).expect("Invalid rule").1)
        .collect();
    let messages = input.next().ok_or_else(invalid_format)?.to_string();
    Ok((messages, rules))
}

#[allow(clippy::map_entry)]
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

#[aoc(day19, part1)]
fn part1((input, rules): &(String, HashMap<usize, Rule>)) -> usize {
    let mut cache = HashMap::new();
    let regexp = format!("^{}$", into_regex_internal(&rules, &mut cache, 0));
    let re = regex::Regex::new(&regexp).unwrap();

    input.lines().filter(|line| re.is_match(&line)).count()
}

fn into_nom_parser(
    rules: &HashMap<usize, Rule>,
    id: usize,
) -> impl FnMut(&str) -> IResult<&str, ()> + '_ {
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
#[aoc(day19, part2)]
fn part2((input, rules): &(String, HashMap<usize, Rule>)) -> usize {
    // assert the assumption about rule 0, 8 and 11 is correct
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
    input.lines().filter(|line| rule0(&line).is_ok()).count()
}
