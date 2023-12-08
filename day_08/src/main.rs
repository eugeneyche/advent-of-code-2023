use std::io;
use std::cmp;
use std::collections::HashMap;

fn main() {
    p2();
}

#[derive(Clone)]
struct Node {
    v: String,
    l: String,
    r: String,
}

struct Network {
    instr: Vec<char>,
    nodes: Vec<Node>,
}

fn read_network_from_stdin() -> Network {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let instr: Vec<char> = lines[0].trim().chars().collect();
    let nodes: Vec<Node> = lines[2..].iter().map(|line| {
        let v: String = line[0..3].into();
        let l: String = line[7..10].into();
        let r: String = line[12..15].into();
        Node { v, l, r }
    }).collect();
    Network { instr, nodes }
}

fn p1() {
    let network = read_network_from_stdin();
    let mut node_map = HashMap::new();
    for n in network.nodes.iter() {
        node_map.insert(n.v.clone(), n.clone());
    }
    let mut pos = "AAA".to_string();
    let mut step = 0;
    while pos != "ZZZ" {
        let n = node_map.get(&pos).unwrap();
        let dir = network.instr[step % network.instr.len()];
        match dir {
            'L' => pos = n.l.clone(),
            'R' => pos = n.r.clone(),
            _ => ()
        } 
        step += 1;
    }
    println!("{}", step);
}

fn count_steps(node_map: &HashMap<String, Node>, instr: &[char], start: &str) -> usize {
    let mut step = 0;
    let mut pos = start.to_string();
    while !pos.ends_with('Z') {
        let n = node_map.get(&pos).unwrap();
        let dir = instr[step % instr.len()];
        match dir {
            'L' => pos = n.l.clone(),
            'R' => pos = n.r.clone(),
            _ => ()
        } 
        step += 1;
    }
    step
} 

fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = (cmp::min(a, b), cmp::max(a, b));
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn p2() {
    let network = read_network_from_stdin();
    let mut node_map = HashMap::new();
    for n in network.nodes.iter() {
        node_map.insert(n.v.clone(), n.clone());
    }
    let ghosts: Vec<String> = network.nodes.iter().filter(|n| n.v.ends_with('A')).map(|n| n.v.clone()).collect();
    let steps: Vec<_> = ghosts.iter().map(|g| count_steps(&node_map, &network.instr, g)).collect();
    println!("{}", steps.iter().fold(1, |acc, &e| lcm(acc, e)));
}