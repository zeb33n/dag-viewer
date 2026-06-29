use crate::{data_types::*, web_print};

use dot_parser::ast::{
    either::Either::{Left, Right},
    Graph,
};

pub struct Model {
    pub nodes: Vec<Node>,
}

pub fn decode_url_encoded_string(encoded_str: &str) -> String {
    encoded_str.replace("%3A", ":").replace("%2F", "/")
}

impl Model {
    fn find_node(nodes: &Vec<Node>, name: &str) -> Option<NodeHandle> {
        for i in 0..nodes.len() {
            if nodes[i].label == name {
                return Some(i);
            }
        }
        None
    }

    fn get_logical_nodes(
        graph: &Graph<(dot_parser::ast::ID<'_>, dot_parser::ast::ID<'_>)>,
    ) -> Vec<Node> {
        let mut r: Vec<Node> = vec![];
        let stmts = &graph.stmts;
        for s in stmts {
            match s {
                dot_parser::ast::Stmt::NodeStmt(node_stmt) => {
                    r.push(Node::new(node_stmt.name().to_string()));
                }
                dot_parser::ast::Stmt::EdgeStmt(edge_stmt) => {
                    let from = match &edge_stmt.from {
                        Left(x) => x,
                        Right(_x) => panic!("not supported!"),
                    };
                    let to = match &edge_stmt.next.to {
                        Left(x) => x,
                        Right(_x) => panic!("not supported!"),
                    };
                    let h_from: NodeHandle = Self::find_node(&r, from.id.as_str()).unwrap();
                    let h_to: NodeHandle = Self::find_node(&r, to.id.as_str()).unwrap();
                    // hack begin - why is it adding duplicates in the first place?
                    let mut b_already_present = false;
                    for i in &r[h_from].dependents {
                        if *i == h_to {
                            b_already_present = true;
                        }
                    }

                    if !b_already_present {
                        r[h_from].dependents.push(h_to);
                    }
                }
                dot_parser::ast::Stmt::AttrStmt(_attr_stmt) => todo!(),
                dot_parser::ast::Stmt::IDEq(_, _) => todo!(),
                dot_parser::ast::Stmt::Subgraph(_subgraph) => todo!(),
            }
        }
        for n in &mut r {
            n.label = decode_url_encoded_string(&n.label).replace("\"", "");
        }
        r
    }

    pub fn new_default() -> Self {
        Self { nodes: vec![] }
    }

    pub fn new(graph: &Graph<(dot_parser::ast::ID<'_>, dot_parser::ast::ID<'_>)>) -> Self {
        Self {
            nodes: dbg!(Self::get_logical_nodes(graph)), // crashes
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
