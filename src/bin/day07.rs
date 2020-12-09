use std::{collections::HashSet, convert::TryFrom};

use anyhow::format_err;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::digit1,
    combinator::value,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};
use petgraph::{graphmap::DiGraphMap, algo::all_simple_paths};

fn colour(input: &str) -> IResult<&str, Colour> {
    map(tuple((alpha1, tag(" "), alpha1)), |(adj, _, colour)| {
        Colour(format!("{} {}", adj, colour))
    })(input)
}

fn contain_rule(input: &str) -> IResult<&str, ContainRule> {
    map(
        tuple((
            map_res(digit1, |s: &str| s.parse::<u8>()),
            tag(" "),
            colour,
            alt((tag(" bags"), tag(" bag"))),
        )),
        |(num, _, colour, _)| ContainRule(num, colour),
    )(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            colour,
            tag(" bags contain "),
            alt((
                value(Vec::new(), tag("no other bags")),
                separated_list1(tag(", "), contain_rule),
            )),
            tag("."),
        )),
        |(colour, _, contain_rules, _)| Rule {
            colour,
            contains: contain_rules,
        },
    )(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Colour(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContainRule(u8, Colour);

#[derive(Debug)]
pub struct Rule {
    pub colour: Colour,
    pub contains: Vec<ContainRule>,
}

impl TryFrom<&str> for Rule {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        rule(value)
            .finish()
            .map(|(_, rule)| rule)
            .map_err(|e| format_err!("Failed to parse rule: {}", e.to_string()))
    }
}

fn part1(rules: &[Rule]) -> usize {
    let mut graph = DiGraphMap::new();
    for rule in rules {
        if rule.contains.is_empty() {
            graph.add_edge(rule.colour.0.as_str(), "no other", 0);
        } else {
            rule.contains.iter().for_each(|c| {
                graph.add_edge(rule.colour.0.as_str(), c.1.0.as_str(), c.0);
            });
        }
    }

    let mut graph = graph.into_graph::<u32>();
    graph.reverse();
    // dbg!(&graph);
    let shiny_gold = graph.node_indices().find(|i| graph[*i] == "shiny gold").unwrap();
    let sink = graph.node_indices().find(|i| graph[*i] == "no other").unwrap();

    dbg!(shiny_gold);
    dbg!(sink);

    let paths = all_simple_paths::<Vec<_>, _>(&graph, sink, shiny_gold, 1, None);

    // dbg!(paths.count());

    let mut set = HashSet::new();
    for path in paths {
        for node in dbg!(path) {
            set.insert(graph[node]);
        }
    }

    // "no other" and "shiny gold" are both included in the set, so account for that in the total
    return set.len() - 2;
}

fn parse_rules(input: &str) -> Result<Vec<Rule>> {
    input
        .lines()
        .map(Rule::try_from)
        .collect::<Result<Vec<Rule>>>()
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let rules = parse_rules(&input)?;
    let count = part1(&rules);
    println!("part 1: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain_rule() {
        assert_eq!(
            contain_rule("3 muted magenta bag"),
            Ok(("", ContainRule(3, Colour("muted magenta".to_string()))))
        );
    }

    #[test]
    fn test_part1() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let rules = parse_rules(input).unwrap();

        assert_eq!(4, part1(&rules));
    }
}
