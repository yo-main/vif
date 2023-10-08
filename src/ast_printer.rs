use crate::ast::Operator;

pub trait AstVisitor2 {
    fn visit_operator(&self, item: &Operator) {}
}

pub struct AstPrinter {}

// impl AstVisitor for AstPrinter {
//     fn visit_operator(&self, item: &Operator) {
//         println!("coucou");
//     }
// }
