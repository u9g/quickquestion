# Quick Question

An experimental language that compiles down to graphql query syntax.

Example:

```
query.File.function {
    parameter.assignment.assignment_to_variable_name @tag(name: "x")
    is_async = $false
    is_generator = $false
    is_getter = $false
    is_setter = $false

    body.statement as ReturnStatement {
        expression as FnCall {
            callee as VarRef {
                name one_of $dont_duplicate_fns
            }

            argument.value as VarRef {
                name = %x
            }
        }
    }


    span {
        start @output(name: "span_start")
        end @output(name: "span_end")
    }
}
```

which compiles to:

```graphql
query {
  File {
    function {
      parameter {
        assignment {
          assignment_to_variable_name @tag(name: "x")
        }
      }
      is_async @filter(op: "=", value: ["$false"])
      is_generator @filter(op: "=", value: ["$false"])
      is_getter @filter(op: "=", value: ["$false"])
      is_setter @filter(op: "=", value: ["$false"])
      body {
        statement {
          ... on ReturnStatement {
            expression {
              ... on FnCall {
                callee {
                  ... on VarRef {
                    name @filter(op: "one_of", value: ["$dont_duplicate_fns"])
                  }
                }
                argument {
                  value {
                    ... on VarRef {
                      name @filter(op: "=", value: ["%x"])
                    }
                  }
                }
              }
            }
          }
        }
      }
      span {
        start @output(name: "span_start")
        end @output(name: "span_end")
      }
    }
  }
}
```