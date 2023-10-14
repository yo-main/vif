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
///     fn visit_a(&mut self, item: &A) {}
///     fn visit_b(&mut self, item: &B) {}
/// }
///
/// impl A {
///     fn accept<V>(&self, visitor: &mut V)
///     where V: &mut MyVisitor {
///         self.visitor.visit_a(self);
///     }
/// }
///
/// impl B {
///     pub fn accept<V>(&self, visitor: &mut V)
///     where V: MyVisitor {
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
                    fn [<visit_$type:lower>](&mut self, item: &$type);
                }
            )*
        }

        $(
            paste::paste! {
                impl $type {
                    pub fn accept<V>(&self, visitor: &mut V)
                    where
                        V: $visitor,
                    {
                        visitor.[<visit_$type:lower>](self);
                    }
                }
            }
        )*
    };
    ($visitor:ident[$($type:ident<$lifetime:tt>),*]) => {
        pub trait $visitor {
            $(
                paste::paste! {
                    fn [<visit_$type:lower>](&mut self, item: &$type);
                }
            )*
        }

        $(
            paste::paste! {
                impl<$lifetime> $type<$lifetime> {
                    pub fn accept<V>(&self, visitor: &mut V)
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
