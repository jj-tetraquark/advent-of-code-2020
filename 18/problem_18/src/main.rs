use std::io::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use std::cell::RefCell;

fn calculate_sequence(sequence :&Vec<&str>, idx : &mut usize) -> i64
{
    let mut op : fn(lvalue: i64, rvalue: i64) -> i64 = |_, rvalue| rvalue;
    let mut lvalue = 0;    
    while *idx < sequence.len()
    {
        *idx += 1;
        match parse_token(sequence[*idx - 1])
        {
            Token::Rvalue(rvalue) => lvalue = op(lvalue, rvalue),
            Token::Add => op = |lvalue, rvalue| lvalue + rvalue,
            Token::Multiply => op = |lvalue, rvalue| lvalue * rvalue,
            Token::OpenParens => lvalue = op(lvalue, calculate_sequence(sequence, idx)),
            Token::CloseParens => return lvalue 
        }
    }
    lvalue
}

enum Token
{
    Rvalue(i64),
    Add,
    Multiply,
    OpenParens,
    CloseParens
}

fn parse_token(token :&str) -> Token
{
    lazy_static!
    {
        static ref OP_REGEX :Regex = Regex::new(r"[\*\+\-/]").unwrap();
    }

    if let Ok(parsed_int) = token.parse::<i64>()
    {
        return Token::Rvalue(parsed_int);
    }
    if OP_REGEX.is_match(token)
    {
        match token.chars().next().unwrap()
        {
            '+' => return Token::Add,
            '*' => return Token::Multiply,
            _ => panic!("This should not happen! {}", token)
        }
    }
    match token.chars().next().unwrap()
    {
        '(' => return Token::OpenParens,
        ')' => return Token::CloseParens,
        _ => panic!("Unparseable token : {}", token)
    }
}

fn split_to_tokens(input :&str) -> Vec<&str>
{
    lazy_static!
    {
        static ref PAREN_REGEX :Regex = 
            Regex::new(r"(\(+)+([\d]+)|([\d]+)(\)+)+").unwrap();
    }
    input
        .split_whitespace()
        .map(|x|
             { 
                 if let Some(parens) = PAREN_REGEX.captures(x)
                 {
                     if let Some(open_parens) = parens.get(1)
                     {
                         let mut tokens :Vec<_>= open_parens
                                                    .as_str()
                                                    .split("")
                                                    .filter(|c| !c.is_empty())
                                                    .collect();

                         tokens.push(parens.get(2).unwrap().as_str());
                         return tokens
                     }
                     else if let Some(close_parens) = parens.get(4)
                     {
                         let mut tokens :Vec<_>= close_parens
                                                    .as_str()
                                                    .split("")
                                                    .filter(|c| !c.is_empty())
                                                    .collect();

                         tokens.insert(0, parens.get(3).unwrap().as_str());
                         return tokens
                     }

                 }
                 return vec![x]
             }).flatten().collect()
}

fn calculate_addition_first(sequence :&Vec<&str>, idx :&mut usize) -> i64
{
    let mut lvalue = 0;
    let mut rvalue = 0;
    let mut is_multiply = false;
    let product_only_seq = RefCell::new(Vec::new());
    let mut op: Box<dyn FnMut(i64, i64) -> i64> = Box::new(move |_, rvalue| rvalue);

    while *idx < sequence.len()
    {
     //   println!("lvalue: {}, token: {}, product_only_seq: {:?}", lvalue, sequence[*idx], product_only_seq.borrow());
        *idx += 1;
        match parse_token(sequence[*idx-1])
        {
            Token::Rvalue(rvalue_) => 
            {
                lvalue = op(lvalue, rvalue_);
                rvalue = rvalue_;
            },
            Token::Add => 
            {
                is_multiply = false;
                op = Box::new(|lvalue, rvalue| lvalue + rvalue)
            },
            Token::Multiply => 
            {
                is_multiply = true;
                op = Box::new(|lvalue, rvalue| 
                                       { 
                                           product_only_seq.borrow_mut().push(lvalue); 
                                           rvalue
                                       })
            },
            Token::OpenParens =>
            {
                rvalue = calculate_addition_first(sequence, idx);
                lvalue = op(lvalue, rvalue)
            },
            Token::CloseParens => 
            {
                break;
            }
        }
    }

    if is_multiply
    {
        product_only_seq.borrow_mut().push(rvalue);
    }
    else
    {
        product_only_seq.borrow_mut().push(lvalue);
    }
    //println!("sequence before calc: {:?}", product_only_seq.borrow());
    let prod = product_only_seq.borrow().iter().product();
    prod
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("Cannot open input file");

    let file_contents :Vec<String> = std::io::BufReader::new(file)
                                                         .lines()
                                                         .filter_map(|line| line.ok())
                                                         .collect();

    let sum :i64 = file_contents
                            .iter()
                            .map(|line| 
                                 {
                                     let mut idx = 0;
                                     calculate_sequence(&split_to_tokens(line), &mut idx)
                                 }).sum();

    println!("Total sum: {}", sum);

    let tests = vec!["1 + (2 * 3) + (4 * (5 + 6))",
                     "2 * 3 + (4 * 5)",
                     "5 + (8 * 3 + 9 + 3 * 4 * 3)",
                     "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                     "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"];
    for test in tests
    {
        let mut idx = 0;
        println!("{} = {}", test, calculate_addition_first(&split_to_tokens(test), &mut idx));
    }

    let sum2 : i64 = file_contents
                        .iter()
                        .map(|line|
                             {
                                 let mut idx = 0;
                                 calculate_addition_first(&split_to_tokens(line), &mut idx)
                             }).sum();

    println!("Part2 sum: {}", sum2);
}
