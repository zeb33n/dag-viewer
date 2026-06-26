use crate::{data_types::*, web_print};

use dot_parser::ast::{
    either::Either::{Left, Right},
    Graph,
};

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub dependents: Vec<LogicalNodeHandle>,
}

pub struct Model {
    pub logical_nodes: Vec<Node>,
}

pub fn decode_url_encoded_string(encoded_str: &str) -> String {
    encoded_str.replace("%3A", ":").replace("%2F", "/")
}

impl Model {
    fn find_node(nodes: &Vec<Node>, name: &str) -> Option<LogicalNodeHandle> {
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
                    r.push(Node {
                        label: node_stmt.name().to_string(),
                        dependents: vec![],
                    });
                }
                dot_parser::ast::Stmt::EdgeStmt(edge_stmt) => {
                    let from = match &edge_stmt.from {
                        Left(x) => x,
                        Right(x) => panic!("not supported!"),
                    };
                    let to = match &edge_stmt.next.to {
                        Left(x) => x,
                        Right(x) => panic!("not supported!"),
                    };
                    let h_from: LogicalNodeHandle = Self::find_node(&r, from.id.as_str()).unwrap();
                    let h_to: LogicalNodeHandle = Self::find_node(&r, to.id.as_str()).unwrap();
                    // hack begin - why is it adding duplicates in the first place?
                    let mut b_already_present = false;
                    for i in &r[h_from].dependents {
                        if *i == h_to {
                            b_already_present = true;
                        }
                    }

                    if (!b_already_present) {
                        r[h_from].dependents.push(h_to);
                    }
                }
                dot_parser::ast::Stmt::AttrStmt(attr_stmt) => todo!(),
                dot_parser::ast::Stmt::IDEq(_, _) => todo!(),
                dot_parser::ast::Stmt::Subgraph(subgraph) => todo!(),
            }
        }
        for n in &mut r {
            n.label = decode_url_encoded_string(&n.label).replace("\"", "");
        }
        r
    }

    pub fn new_default() -> Self {
        Self {
            logical_nodes: vec![],
        }
    }

    pub fn new(graph: &Graph<(dot_parser::ast::ID<'_>, dot_parser::ast::ID<'_>)>) -> Self {
        Self {
            logical_nodes: dbg!(Self::get_logical_nodes(graph)), // crashes
        }
    }

    pub fn get_node(&self, h_node: LogicalNodeHandle) -> &Node {
        &self.logical_nodes[h_node]
    }

    pub fn get_root_node(&self) -> LogicalNodeHandle {
        web_print!("num nodes {}", self.logical_nodes.len());
        let mut roots: Vec<LogicalNodeHandle> = vec![];
        for i in 0..self.logical_nodes.len() {
            let mut depended_on: bool = false;
            for j in 0..self.logical_nodes.len() {
                if j == i {
                    continue;
                }
                let node: &Node = &self.logical_nodes[j];
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
