use crate::{web_print, Node};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenSquare,
    CloseSquare,
    Comma,
    Equal,
    Arrow,
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
                    let next = bytes
                        .next()
                        .ok_or("Could Not Find Closing \"".to_string())?;
                    match next {
                        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => word.push(*next as char),
                        _ => break,
                    }
                }
                Ok(Token::Word(word))
            }
            b';' => continue,
            _ if byte.is_ascii_whitespace() => continue,
            unexpected => Err(format!("Unexpected character {}", *unexpected as char)),
        }?;
        tokens.push(token)
    }
    Ok(tokens)
}

pub fn parse(src: &str) -> Result<Vec<Node>, String> {
    let tokens = tokenise(src)?;
    let mut tokens_iter = tokens.iter().peekable();
    let mut nodes = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    match tokens_iter
        .next()
        .ok_or("expected word token".to_string())?
    {
        Token::Word(v) if v.as_str() == "digraph" => (),
        _ => return Err("unsupported graph type".to_string()),
    };

    match tokens_iter.next().ok_or("expected {".to_string())? {
        Token::OpenBrace => (),
        _ => return Err("Expected {".to_string()),
    }

    loop {
        match tokens_iter.next().ok_or("expected token".to_string())? {
            Token::Word(word) => {
                let parent_handle = insert_and_get_index(&mut seen, &mut nodes, word);
                match tokens_iter.peek() {
                    Some(Token::Arrow) => {
                        tokens_iter.next();
                    }
                    _ => continue,
                }
                let child_handle = if let Token::Word(w) =
                    tokens_iter.next().ok_or("expected token".to_string())?
                {
                    insert_and_get_index(&mut seen, &mut nodes, w)
                } else {
                    return Err("Expected node name".to_string());
                };
                nodes
                    .get_mut(parent_handle)
                    .unwrap()
                    .dependents
                    .push(child_handle);
            }
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
    Ok(nodes)
}

// this is bugged since the order is based on the key order not the order of insertion
// we probably need to just use a hashset and a vec
fn insert_and_get_index<'a>(
    seen: &mut HashSet<&'a str>,
    nodes: &mut Vec<Node>,
    word: &'a str,
) -> usize {
    if seen.insert(word) {
        nodes.push(Node::new(word));
    }
    let mut i = 0;
    for node in nodes.iter() {
        if node.label == word {
            break;
        }
        i += 1;
    }
    i
}
