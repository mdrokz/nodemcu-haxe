use std::collections::HashMap;

use rslua::{
    lexer::Lexer,
    parser::Parser,
    tokens::{Token, TokenType, TokenValue},
};

const LUA_CODE: &'static str = r#"
    function test(a,a1,a)
    end

    function test1
    end

    function test2
    end

    function test3
    end

    function test4
    end

    function test5
    end

    test5()
    test5()
"#;

struct Function {
    t: Token,
    name: Token,
    params: Vec<Token>,
    end: Token
}

fn remove_tokens_until<C>(tokens: &Vec<Token>, c: C)
where
    C: Fn(&Token) -> bool,
{
    let mut a = Vec::<&Token>::new();

    for token in tokens {
        if c(&token) {
            break;
        } else {
            a.push(token);
        }
    }

    println!("{:?}", a);
}

fn main() {
    let mut lexer = Lexer::new();
    let mut parser = Parser::new();
    let mut tokens = lexer.run(LUA_CODE).unwrap();

    let mut f: Vec<Function> = Vec::new();

    let mut tk: Vec<Token> = Vec::new();
    let tk2 = tokens.clone();
    tokens.into_iter().for_each(|t| tk.push(t));

    let functions: Vec<&String> = tk2
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v.t == TokenType::Function {
                if let TokenValue::Str(v) = &tk2[i + 1].value {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    for function in functions {
        let mut map = HashMap::<String, ()>::new();
        let usage: Vec<&Token> = tk2
            .iter()
            .filter(|v| {
                if let TokenValue::Str(s) = &v.value {
                    if function == s && map.contains_key(s) {
                        true
                    } else {
                        map.insert(s.clone(), ());
                        false
                    }
                } else {
                    false
                }
            })
            .collect();
        if usage.len() == 0 {
            tk = tk
                .into_iter()
                .filter_map(|t| {
                    if let TokenValue::Str(ss) = &t.value {
                        if ss == function {
                            None
                        } else {
                            Some(t)
                        }
                    } else {
                        Some(t)
                    }
                })
                .collect();
        }
    }
    // println!("{:?}",tk2);
    println!("{:?}", tk2);
    let block = parser.run(tk).unwrap();

    println!("{:?}", block);
}
