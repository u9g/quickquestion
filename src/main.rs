use std::fs;

use ast::Block;
use pest::Parser;
use pest_derive::Parser;

use crate::printer::print_block;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuickQuestionParser;

mod ast;
mod convert;
mod printer;

fn main() {
    let example = fs::read_to_string("example.qq").unwrap();

    let root_block: Block = QuickQuestionParser::parse(Rule::query, &example)
        .unwrap()
        .next() // query
        .unwrap()
        .into_inner()
        .next() // block
        .unwrap()
        .into();

    // println!("{root_block:#?}")
    let mut output = String::new();
    print_block(&mut output, &root_block, 0);
    println!("{}", output);
}
