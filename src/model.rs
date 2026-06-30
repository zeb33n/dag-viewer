use crate::{data_types::*, parser::parse, web_print};

pub struct Model {
    pub nodes: Vec<Node>,
}

pub fn decode_url_encoded_string(encoded_str: &str) -> String {
    encoded_str.replace("%3A", ":").replace("%2F", "/")
}

impl Model {
    pub fn new_default() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from_source(src: &str) -> Self {
        match parse(src) {
            Ok(ns) => Self { nodes: ns },
            Err(s) => panic!("{}", s),
        }
    }

    pub fn get_node(&self, h_node: NodeHandle) -> &Node {
        &self.nodes[h_node]
    }

    pub fn get_root_node(&self) -> NodeHandle {
        web_print!("num nodes {}", self.nodes.len());
        let mut roots: Vec<NodeHandle> = vec![];
        for i in 0..self.nodes.len() {
            let mut depended_on: bool = false;
            for j in 0..self.nodes.len() {
                if j == i {
                    continue;
                }
                let node: &Node = &self.nodes[j];
                for h_node in &node.dependents {
                    if *h_node == i {
                        depended_on = true;
                        break;
                    }
                }
                if depended_on {
                    break;
                }
            }
            if !depended_on {
                roots.push(i);
            }
        }
        assert!(roots.len() == 1);
        roots[0]
    }
}
