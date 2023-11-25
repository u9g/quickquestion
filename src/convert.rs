use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{
    ast::{Block, BlockMeta, ChainedIdentifier, Directive, Filter, Statement, StatementList},
    Rule,
};

impl<'a> From<Pair<'a, Rule>> for Directive<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        let Rule::directive = value.as_rule() else {
            unreachable!("{:#?}", value.as_rule());
        };

        let mut iter = value.into_inner();

        Self {
            name: iter.next().unwrap().as_str(),
            args: {
                let mut args: HashMap<&'a str, &'a str> = Default::default();

                while iter.len() > 0 {
                    args.insert(iter.next().unwrap().as_str(), iter.next().unwrap().as_str());
                }

                args
            },
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for ChainedIdentifier<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        let Rule::chained_identifier = value.as_rule() else {
            unreachable!("{:#?}", value);
        };

        ChainedIdentifier(value.as_str().split(".").collect::<Vec<_>>())
    }
}

impl<'a> From<Pair<'a, Rule>> for Filter<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        let mut iter = value.into_inner();
        Self {
            identifier: iter.next().unwrap().into(),
            operator: iter.next().unwrap().as_str(),
            filterer: iter.next().map(|x| x.as_str()),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for Statement<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        match value.as_rule() {
            Rule::directives_on_identifier => {
                let mut iter = value.into_inner();
                Statement::IdentifierWithDirectives(
                    iter.next().unwrap().into(),
                    iter.map(Into::into).collect::<Vec<_>>(),
                )
            }
            Rule::block => Statement::Block(Box::new(value.into())),
            Rule::filter => Statement::Filter(value.into()),
            _ => unreachable!("{value:?}"),
        }
        // todo!("{value:?}");
        // let mut iter = value.into_inner();

        // StatementList(iter.map(Into::into).collect::<Vec<_>>())
    }
}

impl<'a> From<Pair<'a, Rule>> for StatementList<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        let Rule::statement_list = value.as_rule() else {
            unreachable!("{:?}", value);
        };

        StatementList(value.into_inner().map(Into::into).collect::<Vec<_>>())
    }
}

impl<'a> From<Pair<'a, Rule>> for Block<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        let mut iter = value.into_inner();

        let identifier: ChainedIdentifier<'a> = iter.next().unwrap().into();

        let body_type = iter.next().unwrap();

        match body_type.as_rule() {
            Rule::directive_prefixed_body => {
                // let mut iter = body_type.into_inner();
                let mut directives: Vec<Directive<'a>> = vec![];
                for item in body_type.into_inner() {
                    match item.as_rule() {
                        Rule::directive => directives.push(item.into()),
                        Rule::statement_list => {
                            return Block {
                                identifier,
                                meta: BlockMeta::Directives(directives),
                                statements: item.into(),
                            };
                        }
                        _ => unreachable!("{:?}", item.as_rule()),
                    }
                }

                unreachable!("there must be a block body in the directive_prefixed_body")
            }
            Rule::casted_body => {
                let mut iter = body_type.into_inner();
                return Block {
                    identifier,
                    meta: BlockMeta::Cast({
                        let mut as_cast = iter.next().unwrap().into_inner();
                        as_cast.next().unwrap().as_str() // casted_to
                    }),
                    statements: iter.next().unwrap().into(),
                };
            }
            _ => unreachable!(),
        }
    }
}
