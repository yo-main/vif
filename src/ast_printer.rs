use std::fmt::format;

use crate::ast::AstVisitor;
use crate::ast::Binary;
use crate::ast::Expr;
use crate::ast::Grouping;
use crate::ast::Literal;
use crate::ast::Operator;
use crate::ast::Unary;

pub struct AstPrinter {
    nodes: Vec<String>,
}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter { nodes: Vec::new() }
    }

    fn get_string(&mut self, expr: &Expr) -> String {
        expr.accept(self);
        format!("{}", self.nodes.join(" "))
    }

    fn parenthesize(&self, expr: String) -> String {
        format!("( {} )", expr)
    }
}

impl AstVisitor for AstPrinter {
    type Item = String;

    fn visit_operator(&mut self, item: &Operator) -> Self::Item {
        format!("{}", item)
    }

    fn visit_literal(&mut self, item: &Literal) -> Self::Item {
        format!("{}", item)
    }

    fn visit_unary(&mut self, item: &Unary) -> Self::Item {
        format!("({} {})", item.operator, item.right.accept(self))
    }
    fn visit_binary(&mut self, item: &Binary) -> Self::Item {
        format!(
            "({} {} {})",
            item.operator,
            item.left.accept(self),
            item.right.accept(self)
        )
    }
    fn visit_grouping(&mut self, item: &Grouping) -> Self::Item {
        format!("(group {})", item.expr.accept(self))
    }

    fn visit_value(&mut self, item: &crate::ast::Value) -> Self::Item {
        format!("{}", item)
    }

    fn visit_expr(&mut self, item: &Expr) -> Self::Item {
        match item {
            Expr::Operator(v) => v.accept(self),
            Expr::Binary(v) => v.accept(self),
            Expr::Unary(v) => v.accept(self),
            Expr::Grouping(v) => v.accept(self),
            Expr::Literal(v) => v.accept(self),
            Expr::Value(v) => v.accept(self),
        }
    }
}

#[cfg(test)]
mod test {
    use super::AstPrinter;
    use crate::ast::{self, Unary};
    use crate::tokens::{Token, TokenType};

    #[test]
    fn nominal_case() {
        let token_one = Token::new(TokenType::Integer(1), 1);
        let token_two = Token::new(TokenType::Integer(2), 1);
        let token_minus = Token::new(TokenType::Minus, 1);
        let token_star = Token::new(TokenType::Star, 1);
        let token_parent_left = Token::new(TokenType::LeftParen, 1);
        let token_parent_right = Token::new(TokenType::RightParen, 1);

        let expr_one = ast::Expr::Literal(ast::Literal::new(token_one));
        let expr_two = ast::Expr::Literal(ast::Literal::new(token_two));
        let expr_minus_one = ast::Expr::Unary(Unary::new(token_minus, Box::new(expr_one)));
        let expr_group_two = ast::Expr::Grouping(ast::Grouping::new(
            token_parent_left,
            Box::new(expr_two),
            token_parent_right,
        ));

        let binary = ast::Binary {
            left: Box::new(expr_minus_one),
            operator: token_star,
            right: Box::new(expr_group_two),
        };

        let mut printer = AstPrinter::new();
        let expr = ast::Expr::Binary(binary);

        assert_eq!(
            printer.get_string(&expr),
            "( * ( - 1 ) ( group 2 ) )".to_owned()
        );
    }
}
