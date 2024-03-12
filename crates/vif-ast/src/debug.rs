use treeline::Tree;
use vif_objects::ast::Assert;
use vif_objects::ast::Assign;
use vif_objects::ast::Binary;
use vif_objects::ast::Call;
use vif_objects::ast::Condition;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::Grouping;
use vif_objects::ast::Logical;
use vif_objects::ast::LoopKeyword;
use vif_objects::ast::Return;
use vif_objects::ast::Stmt;
use vif_objects::ast::Unary;
use vif_objects::ast::Value;
use vif_objects::ast::Variable;
use vif_objects::ast::While;

struct Node {
    name: String,
    kind: String,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.name)
    }
}

impl Node {
    fn new(name: &str, kind: &str) -> Self {
        Node {
            name: name.to_owned(),
            kind: kind.to_owned(),
        }
    }
}

pub fn print_ast_tree(function: &Function) {
    let tree = print_function(function);
    println!("{tree}");
}

fn print_function(function: &Function) -> Tree<Node> {
    Tree::new(
        Node::new(&function.name, "function"),
        function.body.iter().map(print_stmt).collect(),
    )
}

fn print_stmt(stmt: &Stmt) -> Tree<Node> {
    match stmt {
        Stmt::Expression(e) => print_expr(e),
        Stmt::Var(v) => print_var(v),
        Stmt::Function(f) => print_function(f),
        Stmt::Block(b) => print_block(b),
        Stmt::Condition(c) => print_condition(c),
        Stmt::While(w) => print_while(w),
        Stmt::Return(r) => print_return(r),
        Stmt::Assert(a) => print_assert(a),
    }
}

fn print_expr(expr: &Expr) -> Tree<Node> {
    match &expr.body {
        ExprBody::Binary(b) => print_binary(&b),
        ExprBody::Unary(u) => print_unary(&u),
        ExprBody::Grouping(g) => print_grouping(&g),
        ExprBody::Value(v) => print_value(&v),
        ExprBody::LoopKeyword(l) => print_loop(&l),
        ExprBody::Assign(a) => print_assign(&a),
        ExprBody::Call(c) => print_call(&c),
        ExprBody::Logical(l) => print_logical(&l),
    }
}

fn print_binary(binary: &Binary) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{}", binary.operator), "binary"),
        vec![print_expr(&binary.left), print_expr(&binary.right)],
    )
}

fn print_unary(unary: &Unary) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{}", unary.operator), "unary"),
        vec![print_expr(&unary.right)],
    )
}

fn print_grouping(grouping: &Grouping) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{} {}", grouping.left, grouping.right), "grouping"),
        vec![print_expr(&grouping.expr)],
    )
}

fn print_value(value: &Value) -> Tree<Node> {
    Tree::root(Node::new(&format!("{}", value), "value"))
}

fn print_loop(r#loop: &LoopKeyword) -> Tree<Node> {
    Tree::root(Node::new(&format!("{}", r#loop), "keyword"))
}

fn print_assign(assign: &Assign) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{}", assign.name), "assign"),
        vec![print_expr(&assign.value)],
    )
}

fn print_call(call: &Call) -> Tree<Node> {
    let mut callee = print_expr(&call.callee);
    call.arguments.iter().for_each(|e| {
        callee.push(print_expr(&e));
    });

    Tree::new(Node::new("call", ""), vec![callee])
}

fn print_logical(logical: &Logical) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{}", logical.operator), "logical"),
        vec![print_expr(&logical.left), print_expr(&logical.right)],
    )
}

fn print_var(var: &Variable) -> Tree<Node> {
    Tree::new(
        Node::new(&format!("{}", var.name), "variable"),
        vec![print_expr(&var.value)],
    )
}

fn print_block(block: &Vec<Stmt>) -> Tree<Node> {
    Tree::new(
        Node::new("block", ""),
        block.iter().map(print_stmt).collect(),
    )
}

fn print_condition(condition: &Condition) -> Tree<Node> {
    let mut leaves = vec![print_stmt(&condition.then)];
    if condition.r#else.is_some() {
        leaves.push(print_stmt(condition.r#else.as_ref().unwrap()))
    }
    Tree::new(
        Node::new(&format!("{}", condition.expr), "condition"),
        leaves,
    )
}

fn print_while(r#while: &While) -> Tree<Node> {
    let mut cond = print_expr(&r#while.condition);
    cond.push(print_stmt(&r#while.body));

    Tree::new(Node::new("while", "loop"), vec![cond])
}

fn print_return(r#return: &Return) -> Tree<Node> {
    Tree::new(Node::new("return", ""), vec![print_expr(&r#return.value)])
}

fn print_assert(assert: &Assert) -> Tree<Node> {
    Tree::new(Node::new("assert", ""), vec![print_expr(&assert.value)])
}
