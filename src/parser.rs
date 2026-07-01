use std::{iter::Peekable, slice::Iter};

use crate::{
    data_types::{Line, Path},
    is_graphviz_layout, web_print, Node, VecF2,
};

#[derive(Debug)]
pub enum Token {
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
            b';' => continue,
            _ if byte.is_ascii_whitespace() => continue,
            unexpected => Err(format!("Unexpected character {}", *unexpected as char)),
        }?;
        tokens.push(token)
    }
    Ok(tokens)
}

fn tokenise_keywords(word: String) -> Token {
    match word.as_str() {
        "pos" => Token::Pos,
        "digraph" => Token::Digraph,
        "graph" => Token::Graph,
        "node" => Token::Node,
        _ => Token::Word(word),
    }
}

pub fn parse(src: &str) -> Result<(Vec<Node>, Vec<Path>), String> {
    let tokens = tokenise(src)?;
    let mut tokens_iter = tokens.iter().peekable();
    let mut nodes = Vec::new();
    let mut paths = Vec::new();
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
                Some(Token::Arrow) => parse_edge(&mut tokens_iter, &mut nodes, &mut paths, word)?,
                _ => parse_node(&mut tokens_iter, &mut nodes, word)?,
            },
            Token::OpenSquare => loop {
                match tokens_iter.next() {
                    Some(Token::CloseSquare) => break,
                    None => return Err("expected ]".to_string()),
                    _ => (),
                }
            },
            Token::CloseBrace => break,
            _ => return Err("Unexpected token".to_string()),
        }
    }
    Ok((nodes, paths))
}

fn parse_node(
    tokens_iter: &mut Peekable<Iter<Token>>,
    nodes: &mut Vec<Node>,
    parent_name: &str,
) -> Result<(), String> {
    let handle = insert_and_get_index(nodes, parent_name);
    let pos_vec = parse_attributes(tokens_iter)?;
    if !is_graphviz_layout() {
        return Ok(());
    }
    if pos_vec.len() == 0 {
        return Err("expected pos attributte".to_string());
    }
    nodes.get_mut(handle).unwrap().position = pos_vec.get(0).unwrap().to_owned();
    Ok(())
}

fn parse_edge(
    tokens_iter: &mut Peekable<Iter<Token>>,
    nodes: &mut Vec<Node>,
    paths: &mut Vec<Path>,
    parent_name: &str,
) -> Result<(), String> {
    let parent_handle = insert_and_get_index(nodes, parent_name);
    tokens_iter.next();
    let child_handle = match tokens_iter.next().ok_or("expected token".to_string())? {
        Token::Word(w) => Ok(insert_and_get_index(nodes, w)),
        _ => Err("Expected node name".to_string()),
    }?;
    nodes
        .get_mut(parent_handle)
        .unwrap()
        .dependents
        .push(child_handle);
    let pos_vec = parse_attributes(tokens_iter)?;
    let mut path = Path::new(child_handle, parent_handle);
    if !is_graphviz_layout() {
        return Ok(());
    }
    if pos_vec.len() == 0 {
        return Err("expected pos attributte".to_string());
    }
    for w in pos_vec.windows(2) {
        path.line_segments
            .push(Line::new(w[0].to_owned(), w[1].to_owned()))
    }
    paths.push(path);

    Ok(())
}

fn parse_attributes(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<VecF2>, String> {
    match tokens_iter.peek() {
        Some(Token::OpenSquare) => {
            tokens_iter.next();
        }
        _ => return Ok(vec![]),
    }
    let mut out = vec![];
    loop {
        match tokens_iter.next().ok_or("expected token ]".to_string())? {
            Token::Pos => {
                if !matches!(tokens_iter.next(), Some(Token::Equal)) {
                    return Err(format!("expected ="));
                }
                let coords_str = match tokens_iter.next() {
                    Some(Token::Word(w)) => w,
                    _ => return Err("expected coordiantes".to_string()),
                };
                let stripped_coords_str = coords_str.replace("e,", "");
                let parts: Vec<&str> = stripped_coords_str.split(' ').collect();
                for part in parts.iter() {
                    let xy = part.split_once(',').ok_or("expected ,".to_string())?;
                    let x: f32 = xy.0.parse().map_err(|e| format!("{:?} {}", e, xy.0))?;
                    let y: f32 = xy.1.parse().map_err(|e| format!("{:?} {}", e, xy.1))?;
                    out.push(VecF2 { x, y });
                }
            }
            Token::CloseSquare => break,
            _ => (),
        }
    }
    Ok(out)
}

fn insert_and_get_index(nodes: &mut Vec<Node>, word: &str) -> usize {
    for (i, node) in nodes.iter().enumerate() {
        if node.label == word {
            return i;
        }
    }
    nodes.push(Node::new(word));
    nodes.len() - 1
}
