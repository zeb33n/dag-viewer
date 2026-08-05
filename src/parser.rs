use std::{iter::Peekable, slice::Iter};

use crate::{
    data_types::{Line, Path},
    is_graphviz_layout, Node, VecF2,
};

#[derive(Debug)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenSquare,
    CloseSquare,
    Comma,
    Equal,
    Arrow,
    Pos,
    Node,
    Graph,
    Digraph,
    Label,
    Link,
    Word(String),
}

fn tokenise(src: &str) -> Result<Vec<Token>, String> {
    let mut bytes = src.as_bytes().iter().peekable();
    let mut tokens = vec![];
    loop {
        let byte = match bytes.next() {
            Some(v) => v,
            None => break,
        };
        let token = match byte {
            b'{' => Ok(Token::OpenBrace),
            b'}' => Ok(Token::CloseBrace),
            b'[' => Ok(Token::OpenSquare),
            b']' => Ok(Token::CloseSquare),
            b',' => Ok(Token::Comma),
            b'=' => Ok(Token::Equal),
            b'-' => match bytes.next() {
                Some(v) if v == &b'>' => Ok(Token::Arrow),
                _ => Err("expected >".to_string()),
            },
            b'"' => {
                let mut word = String::new();
                loop {
                    let next = bytes
                        .next()
                        .ok_or("Could Not Find Closing \"".to_string())?;
                    match next {
                        b'"' => break,
                        b'\\' => handle_backslash(&mut bytes),
                        v => word.push(*v as char),
                    }
                }
                Ok(Token::Word(word))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                let mut word = (*byte as char).to_string();
                loop {
                    match bytes.peek() {
                        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'.') => {
                            let next = bytes.next().unwrap();
                            word.push(*next as char)
                        }
                        _ => break,
                    }
                }
                Ok(tokenise_keywords(word))
            }
            b'\\' => continue,
            b';' => continue,
            _ if byte.is_ascii_whitespace() => continue,
            unexpected => Err(format!("Unexpected character {}", *unexpected as char)),
        }?;
        tokens.push(token)
    }
    Ok(tokens)
}

fn handle_backslash<'a>(bytes: &mut Peekable<Iter<'a, u8>>) {
    match bytes.peek() {
        Some(byte) if **byte == b'\n' => {
            bytes.next();
        }
        _ => return,
    }
}

fn tokenise_keywords(word: String) -> Token {
    match word.as_str() {
        "pos" => Token::Pos,
        "digraph" => Token::Digraph,
        "graph" => Token::Graph,
        "node" => Token::Node,
        "dv_label" => Token::Label,
        "dv_link" => Token::Link,
        _ => Token::Word(word),
    }
}

struct NodeStatement {
    id: String,
    label: Option<String>,
    coord: Option<VecF2>,
    link: Option<String>,
}

struct EdgeStatement {
    from: String,
    to: String,
    lines: Option<Vec<Line>>,
}

pub fn parse(src: &str) -> Result<(Vec<Node>, Vec<Path>), String> {
    let tokens = tokenise(src)?;
    let mut tokens_iter = tokens.iter().peekable();
    let mut node_statements = Vec::new();
    let mut edge_statements = Vec::new();
    match tokens_iter
        .next()
        .ok_or("expected word token".to_string())?
    {
        Token::Digraph => (),
        _ => return Err("unsupported graph type".to_string()),
    };

    match tokens_iter.next().ok_or("expected {".to_string())? {
        Token::OpenBrace => (),
        _ => return Err("Expected {".to_string()),
    }

    loop {
        match tokens_iter.next().ok_or("expected token".to_string())? {
            Token::Word(word) => match tokens_iter.peek() {
                Some(Token::Arrow) => edge_statements.push(parse_edge(&mut tokens_iter, word)?),
                _ => node_statements.push(parse_node(&mut tokens_iter, word)?),
            },
            Token::OpenSquare => loop {
                match tokens_iter.next() {
                    Some(Token::CloseSquare) => break,
                    None => return Err("expected ]".to_string()),
                    _ => (),
                }
            },
            Token::Node | Token::Graph => continue,
            Token::CloseBrace => break,
            unexpected => return Err(format!("Unexpected token {:?}", unexpected)),
        }
    }

    let mut nodes = Vec::new();
    for ns in node_statements.into_iter() {
        let mut node = Node::new(&ns.id);
        match ns.label {
            Some(l) => node.label = l,
            _ => (),
        }
        node.link = ns.link;
        if is_graphviz_layout() {
            node.position = ns.coord.ok_or(format!("No coordinates for {:?}", ns.id))?;
        }
        nodes.push(node)
    }
    let mut paths = Vec::new();
    for es in edge_statements.into_iter() {
        let to_handle = insert_and_get_index(&mut nodes, &es.to);
        let from_handle = insert_and_get_index(&mut nodes, &es.from);
        let mut path = Path::new(to_handle, from_handle);
        if is_graphviz_layout() {
            path.line_segments = es
                .lines
                .ok_or(format!("No coordinates for {:?} -> {:?}", &es.to, &es.from))?
        }
        nodes[from_handle].dependents.push(to_handle);
        nodes[from_handle].edges.push(paths.len());
        paths.push(path);
    }

    if is_graphviz_layout() {
        adjust_edge_coordinates(&mut nodes, &mut paths);
    }

    Ok((nodes, paths))
}

fn parse_node(
    tokens_iter: &mut Peekable<Iter<Token>>,
    word: &str,
) -> Result<NodeStatement, String> {
    let id = word.to_string();
    match tokens_iter.peek() {
        Some(Token::OpenSquare) => {
            tokens_iter.next();
        }
        _ => {
            return Ok(NodeStatement {
                id,
                label: None,
                coord: None,
                link: None,
            })
        }
    }
    let mut label = None;
    let mut coord = None;
    let mut link = None;
    loop {
        match tokens_iter.next().ok_or("expected token ]".to_string())? {
            Token::Pos if matches!(coord, None) => {
                if !matches!(tokens_iter.next(), Some(Token::Equal)) {
                    return Err(format!("expected ="));
                }
                let coords_str = match tokens_iter.next() {
                    Some(Token::Word(w)) => w,
                    _ => return Err("expected coordinates".to_string()),
                };
                let xy = coords_str.split_once(',').ok_or("expected ,".to_string())?;
                let x: f32 = xy.0.parse().map_err(|e| format!("{:?} {}", e, xy.0))?;
                let y: f32 = xy.1.parse().map_err(|e| format!("{:?} {}", e, xy.1))?;
                coord = Some(VecF2 { x, y });
            }
            Token::Pos => return Err(format!("Position already defined for {:?}", id)),
            Token::Link if matches!(link, None) => {
                if !matches!(tokens_iter.next(), Some(Token::Equal)) {
                    return Err(format!("expected ="));
                }
                link = match tokens_iter.next() {
                    Some(Token::Word(w)) => Some(w.to_owned()),
                    _ => return Err("expected a url".to_string()),
                };
            }
            Token::Link => return Err(format!("Link already defined for {:?}", id)),
            Token::Label if matches!(label, None) => {
                if !matches!(tokens_iter.next(), Some(Token::Equal)) {
                    return Err(format!("expected ="));
                }
                label = match tokens_iter.next() {
                    Some(Token::Word(w)) => Some(w.to_owned()),
                    _ => return Err("expected a string".to_string()),
                };
            }
            Token::Label => return Err(format!("Label already defined for {:?}", id)),
            Token::CloseSquare => break,
            _ => (),
        }
    }
    Ok(NodeStatement {
        id,
        label,
        coord,
        link,
    })
}

fn parse_edge(
    tokens_iter: &mut Peekable<Iter<Token>>,
    parent_name: &str,
) -> Result<EdgeStatement, String> {
    let from = parent_name.to_string();
    tokens_iter.next();
    let to = match tokens_iter.next().ok_or("expected token".to_string())? {
        Token::Word(w) => Ok(w.to_string()),
        _ => Err("Expected node name".to_string()),
    }?;
    match tokens_iter.peek() {
        Some(Token::OpenSquare) => {
            tokens_iter.next();
        }
        _ => {
            return Ok(EdgeStatement {
                from,
                to,
                lines: None,
            })
        }
    }
    let mut lines = None;
    loop {
        match tokens_iter.next().ok_or("expected token ]".to_string())? {
            Token::Pos if matches!(lines, None) => {
                if !matches!(tokens_iter.next(), Some(Token::Equal)) {
                    return Err(format!("expected ="));
                }
                let coords_str = match tokens_iter.next() {
                    Some(Token::Word(w)) => w,
                    _ => return Err("expected coordiantes".to_string()),
                };
                let stripped_coords_str = coords_str.replace("e,", "");
                let parts: Vec<&str> = stripped_coords_str.split(' ').collect();
                let mut coordiantes = vec![];
                for part in parts.iter() {
                    let xy = part.split_once(',').ok_or("expected ,".to_string())?;
                    let x: f32 = xy.0.parse().map_err(|e| format!("{:?} {}", e, xy.0))?;
                    let y: f32 = xy.1.parse().map_err(|e| format!("{:?} {}", e, xy.1))?;
                    coordiantes.push(VecF2 { x, y });
                }
                let mut out = vec![];
                // skip the first pair since this is start to end coords for whole path
                for w in coordiantes.windows(2).skip(1) {
                    out.push(Line::new(w[0].to_owned(), w[1].to_owned()));
                }
                lines = Some(out);
            }
            Token::Pos => return Err("Pos is already defined for edge".to_string()),
            Token::CloseSquare => break,
            _ => (),
        }
    }
    Ok(EdgeStatement { from, to, lines })
}

fn insert_and_get_index(nodes: &mut Vec<Node>, word: &str) -> usize {
    for (i, node) in nodes.iter().enumerate() {
        if node.id == word {
            return i;
        }
    }
    nodes.push(Node::new(word));
    nodes.len() - 1
}

fn adjust_edge_coordinates(nodes: &Vec<Node>, edges: &mut Vec<Path>) {
    for node in nodes.iter() {
        for edge_handle in node.edges.iter() {
            let edge = edges.get_mut(*edge_handle).unwrap();
            edge.line_segments.first_mut().unwrap().a = node.position.to_owned();
            let child = &nodes[edge.to];
            edge.line_segments.last_mut().unwrap().b = child.position.to_owned();
        }
    }
}
