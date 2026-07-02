use std::collections::HashSet;

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
        let mut children = HashSet::new();
        for node in self.nodes.iter() {
            for h in node.dependents.iter() {
                children.insert(h);
            }
        }
        let mut roots = Vec::new();
        for i in 0..self.nodes.len() {
            if !children.contains(&i) {
                roots.push(i);
            }
        }
        if roots.len() != 1 {
            web_print!("too many root");
            for root in roots.iter() {
                web_print!("{:?}", self.nodes[*root]);
            }
            panic!()
        }
        roots[0]
    }
}
