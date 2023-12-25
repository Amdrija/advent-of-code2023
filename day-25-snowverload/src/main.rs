use std::{collections::HashSet, fs};

fn dot_notation(s: &str) -> String {
    let mut result = String::from("graph {\n");

    for line in s.lines() {
        result += "\t";
        result += &line.replace(": ", " -- {");
        result += "}\n";
    }

    result + "}"
}

fn remove_nodes(s: &str, node_names: HashSet<&str>) -> String {
    let mut result = String::new();

    for line in s.lines() {
        if !node_names.contains(line.split(": ").next().unwrap()) {
            result += line;
            result += "\n";
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let removed_nodes = remove_nodes(&input, HashSet::from(["lsv", "dhn", "ptj"]));
    fs::write("removed.dot", dot_notation(&removed_nodes)).unwrap();
}
