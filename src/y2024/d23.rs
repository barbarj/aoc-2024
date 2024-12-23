#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Graph = HashMap<String, HashSet<String>>;

fn get_connections(filename: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string("input/2024/23/".to_owned() + filename).unwrap();
    contents
        .lines()
        .map(|line| (line[0..2].to_string(), line[3..5].to_string()))
        .collect()
}

fn build_graph(edges: Vec<(String, String)>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (s1, s2) in edges {
        graph
            .entry(s1.clone())
            .and_modify(|v| {
                v.insert(s2.clone());
            })
            .or_insert(HashSet::from([s2.clone()]));
        graph
            .entry(s2)
            .and_modify(|v| {
                v.insert(s1.clone());
            })
            .or_insert(HashSet::from([s1]));
    }
    graph
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Group {
    a: String,
    b: String,
    c: String,
}
impl Group {
    fn new(a: String, b: String, c: String) -> Self {
        let mut list = [Some(a), Some(b), Some(c)];
        list.sort();
        Group {
            a: list[0].take().unwrap(),
            b: list[1].take().unwrap(),
            c: list[2].take().unwrap(),
        }
    }
}

fn groups_of_three(graph: &Graph) -> HashSet<Group> {
    let mut groups = HashSet::new();
    for node in graph.keys() {
        let neighbors = graph.get(node).unwrap();
        for n in neighbors.iter() {
            let new_groups = graph
                .get(n)
                .unwrap()
                .intersection(neighbors)
                .map(|n2| Group::new(node.clone(), n.clone(), n2.clone()));
            groups.extend(new_groups);
        }
    }
    groups
}

fn groups_of_three_with_t(filename: &str) -> usize {
    let edges = get_connections(filename);
    let graph = build_graph(edges);
    let groups = groups_of_three(&graph);
    groups
        .iter()
        .filter(|g| g.a.starts_with("t") || g.b.starts_with("t") || g.c.starts_with("t"))
        .count()
}

fn build_group(node: &str, graph: &Graph, seen: &mut HashSet<String>) -> HashSet<String> {
    let mut group = HashSet::from([node.to_string()]);
    seen.insert(node.to_string());
    for (node, neighbors) in graph.iter() {
        if seen.contains(node) {
            continue;
        }
        if neighbors.intersection(&group).count() == group.len() {
            group.insert(node.to_string());
            seen.insert(node.to_string());
        }
    }

    group
}

fn largest_group(filename: &str) -> String {
    let edges = get_connections(filename);
    let graph = build_graph(edges);
    let mut seen = HashSet::new();
    let mut group = None;
    for node in graph.keys() {
        if seen.contains(node) {
            continue;
        }
        let new_group = build_group(node, &graph, &mut seen);
        group = match group {
            None => Some(new_group),
            Some(old_group) => {
                if old_group.len() > new_group.len() {
                    Some(old_group)
                } else {
                    Some(new_group)
                }
            }
        }
    }

    let mut group: Vec<_> = group.unwrap().into_iter().collect();
    group.sort();
    group.join(",")
}

#[cfg(test)]
mod tests {
    use crate::y2024::d23::{build_graph, get_connections, largest_group};

    use super::groups_of_three_with_t;

    #[test]
    fn part1_example() {
        let result = groups_of_three_with_t("example.txt");
        assert_eq!(result, 7);
    }

    #[test]
    fn part1() {
        let result = groups_of_three_with_t("input.txt");
        assert_eq!(result, 1253);
    }

    #[test]
    fn vertice_count() {
        let edges = get_connections("input.txt");
        let graph = build_graph(edges);
        assert_eq!(graph.len(), 520);
    }

    #[test]
    fn part2_example() {
        let result = largest_group("example.txt");
        assert_eq!(result, "co,de,ka,ta");
    }

    #[test]
    fn part2() {
        let result = largest_group("input.txt");
        assert_eq!(result, "ag,bt,cq,da,hp,hs,mi,pa,qd,qe,qi,ri,uq");
    }
}
