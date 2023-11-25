use std::collections::HashMap;

#[derive(Debug)]
pub enum BlockMeta<'a> {
    Directives(Vec<Directive<'a>>),
    Cast(&'a str),
}

#[derive(Debug)]
pub struct Directive<'a> {
    pub name: &'a str,
    pub args: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub identifier: ChainedIdentifier<'a>,
    pub meta: BlockMeta<'a>,
    pub statements: StatementList<'a>,
}

#[derive(Debug)]
pub struct StatementList<'a>(pub Vec<Statement<'a>>);

#[derive(Debug)]
pub enum Statement<'a> {
    Block(Box<Block<'a>>),
    Filter(Filter<'a>),
    IdentifierWithDirectives(ChainedIdentifier<'a>, Vec<Directive<'a>>),
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub identifier: ChainedIdentifier<'a>,
    pub operator: &'a str,
    pub filterer: Option<&'a str>,
}

#[derive(Debug)]
pub struct ChainedIdentifier<'a>(pub Vec<&'a str>);
