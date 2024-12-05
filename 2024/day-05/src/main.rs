use std::{collections::HashMap, fs, path::Path};

struct Page<'a> {
    number: u32,
    ordering_rules: &'a HashMap<u32, Vec<u32>>,
}

fn main() {
    let path = Path::new("input.txt");
    let input_string = fs::read_to_string(path).expect("Cannot solve without input!");
    let mut input = input_string.split("\n\n");

    // `rules` contains all page numbers (`Vec<u32>`) that must come after the key
    let rules = parse_rules(input.next().expect("Input should contain rules."));
    let updates = parse_updates(input.next().expect("Input should contain updates."), &rules);
}

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let mut split = line.split("|");
        let a = split.next().unwrap().parse().unwrap();
        let b = split.next().unwrap().parse().unwrap();
        rules
            .entry(a)
            .and_modify(|pages: &mut Vec<u32>| pages.push(b))
            .or_insert(vec![b]);
    }
    rules
}

fn parse_updates<'a>(input: &str, rules: &'a HashMap<u32, Vec<u32>>) -> Vec<Vec<Page<'a>>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page_number| Page {
                    number: page_number
                        .parse()
                        .expect("Page numbers should be numeric values."),
                    ordering_rules: rules,
                })
                .collect()
        })
        .collect()
}

fn is_ordered(update: &Vec<Page>) -> bool {
    let mut previous = &update[0];
    for page in &update[1..] {
        if page < previous {
            return false;
        }
        previous = page;
    }
    true
}

impl PartialEq for Page<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        } else if self
            .ordering_rules
            .get(&self.number)
            .unwrap()
            .contains(&other.number)
        {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static RULES: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
    static UPDATES: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_is_ordered() {
        let rules = parse_rules(RULES);
        let updates = parse_updates(UPDATES, &rules);
        assert!(is_ordered(&updates[0]));
        assert!(is_ordered(&updates[1]));
        assert!(!is_ordered(&updates[3]));
        assert!(!is_ordered(&updates[4]));
    }

    #[test]
    fn test_parse_rules() {
        let rules = parse_rules("47|53\n97|13\n97|61");
        assert_eq!(rules, HashMap::from([(47, vec![53]), (97, vec![13, 61]),]))
    }
}
