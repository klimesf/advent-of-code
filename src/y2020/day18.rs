use itertools::Itertools;
use std::fs;
use std::iter::once;

pub(crate) fn day18() {
    println!("{}", part_a(fs::read_to_string("input/2020/day18/input.txt").unwrap()));
    println!("{}", part_b(fs::read_to_string("input/2020/day18/input.txt").unwrap()));
}

fn part_a(input: String) -> usize {
    input.lines().map(|line| eval_left_to_right(tokenize(line))).sum()
}

fn part_b(input: String) -> usize {
    input.lines().map(|line| eval_advanced(tokenize(line))).sum()
}

fn eval_left_to_right(tokens: Vec<Token>) -> usize {
    let mut stack = vec![];
    let mut op = Token::Plus;
    for token in tokens {
        match token {
            Token::Num(val) => stack.push(val),
            Token::Plus => op = token,
            Token::Multiply => op = token,
            Token::Brackets(brackets) => stack.push(eval_left_to_right(brackets)),
        }
        if stack.len() == 2 {
            let a = stack[0];
            let b = stack[1];
            stack.clear();
            let c = match op {
                Token::Plus => a + b,
                Token::Multiply => a * b,
                _ => panic!("Unknown operator {:?}", op),
            };
            stack.push(c);
        }
    }
    debug_assert_eq!(stack.len(), 1);
    stack[0]
}

fn eval_advanced(mut tokens: Vec<Token>) -> usize {
    while tokens.len() > 1 {
        let pos = tokens
            .iter()
            .enumerate()
            .filter(|(_, token)| matches!(token, Token::Plus | Token::Multiply))
            .sorted_by(|(a_i, a_token), (b_i, b_token)| match (a_token, b_token) {
                (Token::Plus, Token::Plus) => b_i.cmp(a_i),
                (Token::Multiply, Token::Multiply) => b_i.cmp(a_i),
                (Token::Plus, Token::Multiply) => 1.cmp(&0),
                (Token::Multiply, Token::Plus) => 0.cmp(&1),
                _ => panic!(),
            })
            .last()
            .unwrap()
            .0;

        debug_assert!(pos > 0 && pos < tokens.len() - 1);
        let l = pos - 1;
        let r = pos + 1;

        let lv = match &tokens[l] {
            Token::Num(val) => *val,
            Token::Brackets(brackets) => eval_advanced(brackets.clone()),
            un => panic!("Unexpected left side {:?}", un),
        };
        let rv = match &tokens[r] {
            Token::Num(val) => *val,
            Token::Brackets(brackets) => eval_advanced(brackets.clone()),
            un => panic!("Unexpected right side {:?}", un),
        };
        let res = match &tokens[pos] {
            Token::Plus => Token::Num(lv + rv),
            Token::Multiply => Token::Num(lv * rv),
            un => panic!("Unexpected operator {:?}", un),
        };

        tokens.splice(l..=r, once(res));
    }

    match tokens[0] {
        Token::Num(val) => val,
        _ => panic!("Unexpected tokens[0]: {:?}", tokens[0]),
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut stack: Vec<Vec<Token>> = vec![];
    let mut tokens: Vec<Token> = vec![];

    for c in input.chars() {
        match c {
            '(' => {
                stack.push(tokens.clone());
                tokens = vec![];
            }
            ')' => {
                let token = Token::Brackets(tokens.clone());
                debug_assert_ne!(stack.len(), 0);
                tokens = stack.pop().unwrap();
                tokens.push(token);
            }
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Multiply),
            _ => {
                // The numbers are always just one digit
                if c.is_numeric() {
                    tokens.push(Token::Num(c.to_digit(10).unwrap() as usize));
                }
            }
        }
    }

    tokens
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Token {
    Num(usize),
    Plus,
    Multiply,
    Brackets(Vec<Token>),
}

#[cfg(test)]
mod day18_tests {
    use std::fs;

    use crate::y2020::day18::{part_a, part_b};

    #[test]
    fn test_works() {
        assert_eq!(
            71 + 51 + 26 + 437 + 12240 + 13632,
            part_a(fs::read_to_string("input/2020/day18/test.txt").unwrap())
        );
        assert_eq!(
            231 + 51 + 46 + 1445 + 669060 + 23340,
            part_b(fs::read_to_string("input/2020/day18/test.txt").unwrap())
        );
    }

    #[test]
    fn input_works() {
        assert_eq!(5019432542701, part_a(fs::read_to_string("input/2020/day18/input.txt").unwrap()));
        assert_eq!(70518821989947, part_b(fs::read_to_string("input/2020/day18/input.txt").unwrap()));
    }
}
