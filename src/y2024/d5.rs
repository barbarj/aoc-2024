use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code, clippy::while_let_on_iterator)]
fn parse_input(filename: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let file = File::open("input/2024/5/".to_owned() + filename).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut rules = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('|').map(|x| x.parse::<u32>().unwrap());
        rules.push((parts.next().unwrap(), parts.next().unwrap()));
    }

    let mut updates = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let update = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        updates.push(update);
    }

    (rules, updates)
}

fn construct_rule_order(rule_edges: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> Vec<u32> {
    let update_set: HashSet<u32> = update.iter().cloned().collect();
    let mut pruned_rules: HashMap<u32, HashSet<u32>> = rule_edges
        .iter()
        .filter(|(k, _)| update_set.contains(k))
        .map(|(k, v)| {
            let relevant_vals: HashSet<u32> = v.intersection(&update_set).cloned().collect();
            (*k, relevant_vals)
        })
        .collect();

    let mut order = Vec::new();
    while !pruned_rules.is_empty() {
        let next_val = pruned_rules
            .iter()
            .find(|(_, v)| v.is_empty())
            .map(|(k, _)| *k)
            .unwrap();
        pruned_rules.remove(&next_val);
        pruned_rules.iter_mut().for_each(|(_, v)| {
            v.remove(&next_val);
        });
        order.push(next_val);
    }

    order.reverse();
    order
}

#[allow(dead_code)]
fn updates_middles_sum(filename: &str, valids: bool) -> u32 {
    let (rules, updates) = parse_input(filename);

    let mut rule_edges = HashMap::new();
    for rule in rules {
        rule_edges
            .entry(rule.0)
            .and_modify(|v: &mut HashSet<u32>| {
                v.insert(rule.1);
            })
            .or_insert(HashSet::from([rule.1]));
        rule_edges.entry(rule.1).or_insert(HashSet::new());
    }

    // for each update, check if all pre-reqs have been seen at each number
    let mut middles_sum = 0;
    for update in updates {
        let rule_based_order = construct_rule_order(&rule_edges, &update);
        if valids && rule_based_order == update {
            middles_sum += update[update.len() / 2];
        } else if !valids && rule_based_order != update {
            middles_sum += rule_based_order[rule_based_order.len() / 2];
        }
    }
    middles_sum
}

// correct traversal of the rule graph will provide an ordering.
// - this traversal is any traversal that does not produce cycles

// for part 2 {
//   - convert update to set
//   - using set, prune graph (cloned) to only values present in the set.
//   -

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{parse_input, updates_middles_sum};

    #[test]
    fn part1_example() {
        let result = updates_middles_sum("example.txt", true);
        assert_eq!(result, 143);
    }

    #[test]
    fn part1() {
        let result = updates_middles_sum("input.txt", true);
        assert_eq!(result, 5391);
    }

    #[test]
    fn show_no_unordered_numbers_present() {
        let (rules, updates) = parse_input("input.txt");
        let mut seen = HashSet::new();
        for rule in rules {
            seen.insert(rule.0);
            seen.insert(rule.1);
        }
        let mut contention = true;
        for update in updates {
            if update.iter().any(|x| !seen.contains(x)) {
                contention = false;
                break;
            }
        }
        assert!(contention);
    }

    #[test]
    fn part2_example() {
        let result = updates_middles_sum("example.txt", false);
        assert_eq!(result, 123);
    }

    #[test]
    fn part2() {
        let result = updates_middles_sum("input.txt", false);
        assert_eq!(result, 6142);
    }
}
