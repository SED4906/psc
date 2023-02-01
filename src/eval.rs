use crate::parser::Expression;

fn evaluate(expr: Expression, stack: &mut Vec<Expression>) {
    match expr {
        Expression::Comment => {},
        Expression::Number(_) | Expression::String(_) |
        Expression::SlashIdentifier(_) | Expression::HexString(_) |
        Expression::Array(_) | Expression::Block(_) => stack.push(expr),
        Expression::Identifier(id) => {
            match id.as_str() {
                "mul" => {
                    let a = stack.pop().expect("underflow");
                    let b = stack.pop().expect("underflow");
                    if let Expression::Number(a_value) = a {
                        if let Expression::Number(b_value) = b {
                            stack.push(Expression::Number(a_value * b_value));
                        }
                    }
                }
                "=" => {
                    let value = stack.pop().expect("underflow");
                    match value {
                        Expression::Number(number) => println!("{number}"),
                        Expression::String(string) => println!("{string}"),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}