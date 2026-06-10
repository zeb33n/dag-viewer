use crate::data_types::*;
use dot_parser::ast::{
    either::Either::{Left, Right},
    Graph,
};

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub dependents: Vec<LogicalNodeHandle>,
}

pub struct Line {
    pub a: VecF2,
    pub b: VecF2,
    pub colour: Colour,
}

pub struct Path {
    pub from: LogicalNodeHandle,
    pub to: LogicalNodeHandle,
    pub line_segments: Vec<Line>,
}

pub struct Model {
    pub logical_nodes: Vec<Node>,
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
                    r[h_from].dependents.push(h_to);
                }
                dot_parser::ast::Stmt::AttrStmt(attr_stmt) => todo!(),
                dot_parser::ast::Stmt::IDEq(_, _) => todo!(),
                dot_parser::ast::Stmt::Subgraph(subgraph) => todo!(),
            }
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
}
