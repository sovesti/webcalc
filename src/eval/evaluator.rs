use crate::eval::{
    lexer::{CalcError, Operator, tokenize},
    parser::{Expr, UnaryOperator, parse},
};


pub(crate) fn evaluate_expression(expression: &str) -> Result<f64, CalcError> {

    let tokens = tokenize(expression.to_owned())?;
    let ast = parse(&tokens)?;
    evaluate_ast(&ast)
}

fn evaluate_ast(expr: &Expr) -> Result<f64, CalcError> {
    let value = match expr {
        Expr::NumberInt(value) => *value as f64,
        Expr::NumberFloat(value) => *value,
        Expr::UnaryOp { op, operand } => {
            let operand = evaluate_ast(operand)?;
            match op {
                UnaryOperator::Plus => operand,
                UnaryOperator::Minus => -operand,
            }
        }
        Expr::BinaryOp { op, left, right } => {
            let left = evaluate_ast(left)?;
            let right = evaluate_ast(right)?;
            match op {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => {
                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    left / right
                }
            }
        }
    };

    Ok(value)
}
