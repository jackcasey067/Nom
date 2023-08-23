
use super::ast::{AST, ExprAST};


#[derive(Debug)]
pub struct InterpretError (String);

pub fn interpret_ast(ast: AST) -> Result<(), InterpretError> {
    for declaration in ast.declarations {
        match declaration {
            crate::ast::DeclarationAST::Function { name, block, .. } if name == "main" =>{
                let value = evaluate_expr(block)?;
                println!("Main evaluated to {}", value);
            }
            crate::ast::DeclarationAST::Function { .. } => {
                println!("NotYetImplemented: running code in functions other than main()");
            }
        }
    }

    Ok(())
}

pub fn evaluate_expr(ast: ExprAST) -> Result<i32, InterpretError> {
    match ast {
        ExprAST::Add(left, right, ..) => Ok(evaluate_expr(*left)? + evaluate_expr(*right)?),
        ExprAST::Subtract(left, right, ..) => Ok(evaluate_expr(*left)? - evaluate_expr(*right)?),
        ExprAST::Multiply(left, right, ..) => Ok(evaluate_expr(*left)? * evaluate_expr(*right)?),
        ExprAST::Divide(left, right, ..) => Ok(evaluate_expr(*left)? / evaluate_expr(*right)?),
        ExprAST::Literal(i, ..) => Ok(i),
        ExprAST::Block(statements, opt_expr, ..) => {
            /* This needs some work */
            if statements.len() > 0 {
                println!("NotYetImplemented: {} statement(s) parsed but not run.", statements.len())
            }
            
            if let Some(boxed_expr) = opt_expr {
                evaluate_expr(*boxed_expr)
            }
            else {
                Err(InterpretError("Not yet implemented: Block without final expression".to_string()))
            }
        }
    }
}