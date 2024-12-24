#![allow(dead_code)]

use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum GateKind {
    Identity,
    Or,
    And,
    Xor,
}
impl From<&str> for GateKind {
    fn from(value: &str) -> Self {
        match value {
            "OR" => Self::Or,
            "AND" => Self::And,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gate {
    kind: GateKind,
    inputs: Vec<usize>,
    value: Option<bool>,
}
impl Gate {
    fn new(kind: GateKind, value: Option<bool>) -> Self {
        Gate {
            kind,
            inputs: Vec::new(),
            value,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    pos_lookup: HashMap<String, usize>,
    gates: Vec<Gate>,
}
impl Circuit {
    fn new() -> Self {
        Circuit {
            pos_lookup: HashMap::new(),
            gates: Vec::new(),
        }
    }

    fn swap_gates(&mut self, n1: &str, n2: &str) {
        let pos1 = *self.pos_lookup.get(n1).unwrap();
        let pos2 = *self.pos_lookup.get(n2).unwrap();

        let tmp_kind = self.gates[pos1].kind.clone();
        self.gates[pos1].kind = self.gates[pos2].kind.clone();
        self.gates[pos2].kind = tmp_kind;

        let tmp_inputs = self.gates[pos1].inputs.clone();
        self.gates[pos1].inputs = self.gates[pos2].inputs.clone();
        self.gates[pos2].inputs = tmp_inputs;
    }

    fn add_gate(&mut self, gate: Gate, name: String) {
        let insert_pos = self.gates.len();
        self.gates.push(gate);
        self.pos_lookup.insert(name, insert_pos);
    }

    fn get_gate(&self, name: &str) -> &Gate {
        let pos = self.pos_lookup.get(name).unwrap();
        self.gates.get(*pos).unwrap()
    }

    fn get_gate_mut(&mut self, name: &str) -> &mut Gate {
        let pos = self.pos_lookup.get(name).unwrap();
        self.gates.get_mut(*pos).unwrap()
    }

    fn evaluate_gate_name(&mut self, name: &str) -> bool {
        let pos = *self.pos_lookup.get(name).unwrap();
        self.evaluate_gate(pos)
    }

    fn evaluate_gate(&mut self, gate_pos: usize) -> bool {
        assert!(gate_pos < self.gates.len());
        let gate = &self.gates[gate_pos];
        if let Some(val) = gate.value {
            return val;
        }
        assert_eq!(gate.inputs.len(), 2);
        let pos1 = gate.inputs[0];
        let pos2 = gate.inputs[1];
        let val = match gate.kind {
            GateKind::Identity => unreachable!(),
            GateKind::Or => self.evaluate_gate(pos1) || self.evaluate_gate(pos2),
            GateKind::And => self.evaluate_gate(pos1) && self.evaluate_gate(pos2),
            GateKind::Xor => self.evaluate_gate(pos1) != self.evaluate_gate(pos2),
        };
        let gate = &mut self.gates[gate_pos];
        gate.value = Some(val);
        val
    }
}

fn build_circuit(filename: &str) -> Circuit {
    let contents = fs::read_to_string("input/2024/24/".to_owned() + filename).unwrap();
    let mut big_sections = contents.split("\n\n");
    let mut circuit = Circuit::new();

    // with initial values
    for line in big_sections.next().unwrap().lines() {
        let name = line[0..3].to_string();
        let init_val = match &line[5..6] {
            "1" => true,
            "0" => false,
            _ => unreachable!(),
        };
        let new_gate = Gate::new(GateKind::Identity, Some(init_val));
        circuit.add_gate(new_gate, name);
    }

    let connections = big_sections.next().unwrap();
    // add names
    for line in connections.lines() {
        let mut parts = line.split_whitespace();
        let name1 = parts.next().unwrap().to_string();
        let gate1 = Gate::new(GateKind::Identity, None);
        if !circuit.pos_lookup.contains_key(&name1) {
            circuit.add_gate(gate1, name1);
        }

        parts.next();
        let name2 = parts.next().unwrap().to_string();
        let gate2 = Gate::new(GateKind::Identity, None);
        if !circuit.pos_lookup.contains_key(&name2) {
            circuit.add_gate(gate2, name2);
        }

        parts.next();
        let name3 = parts.next().unwrap().to_string();
        let gate3 = Gate::new(GateKind::Identity, None);
        if !circuit.pos_lookup.contains_key(&name3) {
            circuit.add_gate(gate3, name3);
        }
    }

    // add connections and kinds
    for line in connections.lines() {
        let mut parts = line.split_whitespace();
        let name1 = parts.next().unwrap();
        let kind = parts.next().unwrap();
        let name2 = parts.next().unwrap();
        parts.next();
        let name3 = parts.next().unwrap();

        let pos1 = *circuit.pos_lookup.get(name1).unwrap();
        let pos2 = *circuit.pos_lookup.get(name2).unwrap();
        let gate = circuit.get_gate_mut(name3);
        gate.kind = GateKind::from(kind);
        gate.inputs.push(pos1);
        gate.inputs.push(pos2);
    }

    circuit
}

fn construct_num(circuit: &Circuit, c: char) -> u64 {
    let mut gates: Vec<_> = circuit
        .pos_lookup
        .keys()
        .filter(|k| k.starts_with(c))
        .cloned()
        .collect();
    gates.sort();
    let mut num = 0;
    for (idx, gate_name) in gates.iter().enumerate() {
        let v = circuit
            .get_gate(gate_name)
            .value
            .map(|v| if v { 1 } else { 0 })
            .unwrap();
        num |= v << idx
    }
    num
}

fn evaluate_adder(circuit: &mut Circuit) -> u64 {
    let keys: Vec<_> = circuit.pos_lookup.keys().cloned().collect();
    for k in keys {
        circuit.evaluate_gate_name(&k);
    }
    construct_num(circuit, 'z')
}

fn construct_result(filename: &str) -> u64 {
    let mut circuit = build_circuit(filename);
    evaluate_adder(&mut circuit)
}

#[cfg(test)]
mod tests {
    use crate::y2024::d24::evaluate_adder;

    use super::{build_circuit, construct_num, construct_result};

    #[test]
    fn part1_small_example() {
        let result = construct_result("small_example.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_large_example() {
        let result = construct_result("large_example.txt");
        assert_eq!(result, 2024);
    }

    #[test]
    fn part1() {
        let result = construct_result("input.txt");
        assert_eq!(result, 55544677167336);
    }

    #[test]
    fn input_gate_count() {
        let circuit = build_circuit("input.txt");
        assert_eq!(circuit.gates.len(), 312);
    }

    #[test]
    fn x_num() {
        let circuit = build_circuit("input.txt");
        let x = construct_num(&circuit, 'x');
        assert_eq!(x, 33884112699961);
    }

    #[test]
    fn y_num() {
        let circuit = build_circuit("input.txt");
        let y = construct_num(&circuit, 'y');
        assert_eq!(y, 21587482918575);
    }

    #[test]
    fn expected_result() {
        let mut circuit = build_circuit("input.txt");
        let x = construct_num(&circuit, 'x');
        let y = construct_num(&circuit, 'y');
        let expected = x + y;
        circuit.swap_gates("kth", "z12");
        circuit.swap_gates("gsd", "z26");
        circuit.swap_gates("tbt", "z32");
        circuit.swap_gates("vpm", "qnf");
        let result = evaluate_adder(&mut circuit);
        println!("{:?}", circuit.get_gate("nwm").value);
        println!("{expected:#b}");
        println!("{result:#b}");
        assert_eq!(result, expected);
        let mut swappable = ["kth", "z12", "gsd", "z26", "tbt", "z32", "vpm", "qnf"];
        swappable.sort();
        let output = swappable.join(",");
        assert_eq!(output, "gsd,kth,qnf,tbt,vpm,z12,z26,z32");
    }
}
