use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;


use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::context;
use nom::multi::{separated_list0, many0};
use nom::sequence::delimited;
use nom::IResult;

use crate::ast::{Expression, Statement};
use crate::parsers::tokens::{parse_identifier, get_identifier};


use super::expressions::expression;

use super::statements::{parse_return_statement, parse_let_statement, parse_print_statement};



impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "Identifier({})", s),
            Expression::Number(n) => write!(f, "Number({})", n),
            Expression::String(s) => write!(f, "String({})", s),
            Expression::Char(c) => write!(f, "Char({})", c),
            Expression::Infix(op, left, right) => write!(f, "Infix({} {} {})", op, left, right),
            Expression::Call(ident, args) => write!(f, "Call({} {})", ident, args.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}

fn parse_function_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, _) = ws(tag("fn"))(input)?;
    //println!("found fn");
    let (input, _) = multispace0(input)?;
    let (input, ident) = parse_identifier(input)?;
   //println!("fn ident: {:?}", ident);
    let (input, _) = multispace0(input)?;
    let (input, params) = delimited(
        tag("("), 
        separated_list0(tag(","), parse_identifier), 
        tag(")")
    )(input)?;
    //println!("params: {:?}", params);
    let (input, _) = multispace0(input)?;
    //println!("checking for body: {:?}", input);

    let (input, _left_brace) = tag("{")(input)?;
    //println!("left_brace: {:?}", left_brace);
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(ws(parse_statement))(input)?;
    //println!("body: {:?}", body);
    let (input, _) = multispace0(input)?;
    let (input, _right_brace) = tag("}")(input)?;

    //let (input, body) = delimited(
    //    tag("{"), many0(ws(parse_statement)), tag("}"))
    //    (input)?;
//
    //println!("body: {:?}", body);
    let params = params.into_iter().map(|expr| {
        if let Expression::Identifier(s) = expr {
            s
        } else {
            panic!("Expecting identifier in function parameter list")
        }
    }).collect();
    
    match get_identifier(ident) {
        Some(s) => Ok((input, Statement::Function(s, params, body))),
        None => panic!("Expecting identifier in function declaration"),
    }
}


fn parse_expr_statement(input: &str) -> IResult<&str, Statement> {
    let (input, expr) = expression(input)?;
    Ok((input, Statement::Expr(expr)))
}

pub fn ws<'a, T, F>(parser: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    //move |input: &'a str| {
    //    let (input, _) = statement_delimiter(input)?;
    //    parser(input)
    //}
    move |input: &'a str| {
        let (input, _) = opt(multispace0)(input)?;
        let (input, _) = opt(tag("\n"))(input)?;
        let (input, _) = opt(tag("\t"))(input)?;
        parser(input)
    }
}


pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    context(
        "statement",
        alt((
            parse_function_declaration,
            parse_print_statement,
            parse_let_statement,
            parse_expr_statement,
            parse_return_statement,
        )),
    )(input).map(|(input, statement)| {
        //println!("parse_input!: {:?}", input);
        //println!("parse_statement: {:?}", statement);
        (input, statement)
    })
}