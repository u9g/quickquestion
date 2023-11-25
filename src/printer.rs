use crate::ast::{Block, BlockMeta, ChainedIdentifier, Statement};

const INDENT_CHARS: &str = "  ";

pub fn print_block(output: &mut String, block: &Block, indent: usize) {
    let mut nesting_levels_to_clear = 0;

    let mut iter = block.identifier.0.iter().peekable();

    while let Some(part) = iter.next() {
        output.push_str(&INDENT_CHARS.repeat(indent + nesting_levels_to_clear));
        output.push_str(part);
        if iter.peek().is_none() {
            if let BlockMeta::Directives(directives) = &block.meta {
                if directives.len() > 0 {
                    output.push(' ');
                }
                output.push_str(
                    &directives
                        .iter()
                        .map(|x| {
                            format!(
                                "@{}{}",
                                x.name,
                                if x.args.len() > 0 {
                                    format!(
                                        "({})",
                                        x.args
                                            .iter()
                                            .map(|arg| format!("{}: {}", arg.0, arg.1))
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    )
                                } else {
                                    "".into()
                                }
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                );
            }
        }
        output.push(' ');
        output.push('{');
        if iter.peek().is_none() {
            if let BlockMeta::Cast(cast) = &block.meta {
                nesting_levels_to_clear += 1;
                output.push('\n');
                output.push_str(&INDENT_CHARS.repeat(indent + nesting_levels_to_clear));
                output.push_str(&format!("... on {}", cast));
                output.push(' ');
                output.push('{');
            }
        }
        output.push('\n');
        nesting_levels_to_clear += 1;
    }

    fn print_directive(
        output: &mut String,
        indent: usize,
        name: &ChainedIdentifier,
        directive_printed: String,
    ) {
        let mut local_indent = indent;
        let mut iter = name.0.iter().peekable();

        while let Some(part) = iter.next() {
            output.push_str(&INDENT_CHARS.repeat(local_indent));
            output.push_str(part);
            output.push(' ');

            if iter.peek().is_some() {
                output.push('{');
            } else {
                output.push_str(&directive_printed);
            }
            output.push('\n');
            local_indent += 1;
        }

        local_indent -= 2;

        for _ in 0..name.0.len() - 1 {
            output.push_str(&INDENT_CHARS.repeat(local_indent));
            output.push('}');
            output.push('\n');
            local_indent -= 1;
        }
    }

    for statement in &block.statements.0 {
        match statement {
            Statement::Block(block) => print_block(output, block, nesting_levels_to_clear + indent),
            Statement::Filter(filter) => print_directive(
                output,
                nesting_levels_to_clear + indent,
                &filter.identifier,
                format!(
                    "@filter(op: \"{}\"{})",
                    filter.operator,
                    if let Some(x) = filter.filterer {
                        format!(", value: [\"{}\"]", x)
                    } else {
                        "".into()
                    }
                )
                .to_string(),
            ),
            Statement::IdentifierWithDirectives(ident, directives) => print_directive(
                output,
                nesting_levels_to_clear + indent,
                &ident,
                directives
                    .iter()
                    .map(|x| {
                        format!(
                            "@{}{}",
                            x.name,
                            if x.args.len() > 0 {
                                format!(
                                    "({})",
                                    x.args
                                        .iter()
                                        .map(|arg| format!("{}: {}", arg.0, arg.1))
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                )
                            } else {
                                "".into()
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
        }
    }

    for _ in 0..nesting_levels_to_clear {
        nesting_levels_to_clear -= 1;
        output.push_str(&INDENT_CHARS.repeat(nesting_levels_to_clear + indent));
        output.push('}');
        output.push('\n');
    }
}
