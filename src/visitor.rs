/// Implement a Visitor pattern which allows to easily add logic on top on
/// other structures.
///
/// The macro achieves this by creating Visitor Trait and implementing it
/// to its visited objects.
///
/// Example:
/// ```rust
/// struct A {}
/// struct B {}
///
/// Visitor!(MyVisitor[A, B]);
///
/// // this creates
/// pub trait MyVisitor {
///     fn visit_a(&self, item: &A) {}
///     fn visit_b(&self, item: &B) {}
/// }
///
/// impl A {
///     fn accept<V>(&self, visitor: V) {
///         self.visitor.visit_a(self);
///     }
/// }
///
/// impl B {
///     fn accept<V>(&self, visitor: V) {
///         self.visitor.visit_b(self);
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! Visitor {
    ($visitor:ident[$($type:ident),*]) => {
        pub trait $visitor {
            $(
                paste::paste! {
                    fn [<visit_$type:lower>](&self, item: &$type) {}
                }
            )*
        }

        $(
            paste::paste! {
                impl $type {
                    fn accept<V>(&self, visitor: V)
                    where
                        V: $visitor,
                    {
                        visitor.[<visit_$type:lower>](self);
                    }
                }
            }
        )*
    };
}
