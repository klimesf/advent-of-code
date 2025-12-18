use std::fs;

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
    let mut stack: Vec<(Vec<usize>, Token)> = vec![];
    let mut operands: Vec<usize> = vec![];
    let mut operator: Token = Token::Plus;

    for token in tokens {
        match token {
            Token::Num(val) => {
                operands.push(val);
            }
            Token::Plus => {
                operator = token;
            }
            Token::Multiply => {
                operator = token;
            }
            Token::OpenBracket => {
                stack.push((operands.clone(), operator));
                operands = vec![];
            }
            Token::CloseBracket => {
                if operands.len() != 1 || stack.len() < 1 {
                    panic!("Unexpected close bracket");
                }

                let a = operands.pop().unwrap();
                let (ops, oper) = stack.pop().unwrap();
                operands = ops;
                operator = oper;
                operands.push(a);
            }
        }

        if operands.len() == 2 {
            match operator {
                Token::Plus => {
                    let a = operands.pop().unwrap();
                    let b = operands.pop().unwrap();
                    operands.push(a + b);
                }
                Token::Multiply => {
                    let a = operands.pop().unwrap();
                    let b = operands.pop().unwrap();
                    operands.push(a * b);
                }
                _ => panic!("Unexpected operator {:?}", operator),
            }
        }
    }

    if operands.len() != 1 {
        panic!("Unexpected end of tokens");
    }
    operands.pop().unwrap()
}

fn eval_advanced(mut tokens: Vec<Token>) -> usize {
    while tokens.len() > 1 {
        let mut max = 0;
        let mut max_pos = usize::MAX;
        let mut height = 0;

        for i in 0..tokens.len() {
            let token = tokens[i];
            match token {
                Token::Num(_) => {}
                Token::Plus => {
                    let val = height * 10 + 2;
                    if val > max {
                        max = val;
                        max_pos = i;
                    }
                }
                Token::Multiply => {
                    let val = height * 10 + 1;
                    if val > max {
                        max = val;
                        max_pos = i;
                    }
                }
                Token::OpenBracket => height += 1,
                Token::CloseBracket => height -= 1,
            }
        }

        let l = tokens[max_pos - 1];
        let op = tokens[max_pos];
        let r = tokens[max_pos + 1];
        let res: Token;

        match (l, r) {
            (Token::Num(lv), Token::Num(rv)) => match op {
                Token::Plus => {
                    res = Token::Num(lv + rv);
                }
                Token::Multiply => {
                    res = Token::Num(lv * rv);
                }
                _ => panic!("Unexpected operator {:?}", op),
            },
            _ => panic!("Unexpected tokens {:?} {:?}", l, r),
        }

        let mut new_tokens = vec![];

        for i in 0..max_pos - 1 {
            new_tokens.push(tokens[i]);
        }
        new_tokens.push(res);

        for i in max_pos + 2..tokens.len() {
            new_tokens.push(tokens[i]);
        }
        tokens = new_tokens;

        // Remove braces surrounding a lone number
        loop {
            let mut found = false;
            new_tokens = vec![];
            for i in 1..tokens.len() - 1 {
                let l = tokens[i - 1];
                let r = tokens[i + 1];
                match (l, r) {
                    (Token::OpenBracket, Token::CloseBracket) => {
                        for j in 0..i - 1 {
                            new_tokens.push(tokens[j]);
                        }
                        new_tokens.push(tokens[i]);
                        for j in i + 2..tokens.len() {
                            new_tokens.push(tokens[j]);
                        }
                        found = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !found {
                break;
            }
            tokens = new_tokens;
        }
    }
    match tokens[0] {
        Token::Num(val) => val,
        _ => panic!("Unexpected token left {:?}", tokens[0]),
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for c in input.chars() {
        match c {
            '(' => tokens.push(Token::OpenBracket),
            ')' => tokens.push(Token::CloseBracket),
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Token {
    Num(usize),
    Plus,
    Multiply,
    OpenBracket,
    CloseBracket,
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
