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
///     type Item;
///     fn visit_a(&mut self, item: &A) -> Self::Item {}
///     fn visit_b(&mut self, item: &B) -> Self::Item {}
/// }
///
/// impl A {
///     fn accept<V>(&self, visitor: &mut V) -> V::Item
///     where V: &mut MyVisitor {
///         self.visitor.visit_a(self)
///     }
/// }
///
/// impl B {
///     pub fn accept<V>(&self, visitor: &mut V) -> V::Item
///     where V: MyVisitor {
///         self.visitor.visit_b(self)
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! Visitor {
    ($visitor:ident[$($type:ident),*]) => {
        pub trait $visitor {
            type Item;
            $(
                paste::paste! {
                     fn [<visit_$type:lower>](&mut self, item: &$type) -> Self::Item;
                }
            )*
        }

        $(
            paste::paste! {
                impl $type {
                    pub fn accept<V>(&self, visitor: &mut V) -> V::Item
                    where
                        V: $visitor,
                    {
                        visitor.[<visit_$type:lower>](self)
                    }
                }
            }
        )*
    };
    ($visitor:ident<$lifetime:tt>[$($type:ident),*]) => {
        pub trait $visitor<$lifetime> {
            type Item;
            $(
                paste::paste! {
                     fn [<visit_$type:lower>](&mut self, item: &$lifetime $type) -> Self::Item;
                }
            )*
        }

        $(
            paste::paste! {
                impl<'s, $lifetime> $type {
                    pub fn accept<V>(&'s self, visitor: &$lifetime mut V) -> V::Item
                    where
                        V: $visitor<$lifetime>,
                        's: $lifetime
                    {
                        visitor.[<visit_$type:lower>](self)
                    }
                }
            }
        )*
    };
    ($visitor:ident[$($type:ident<$lifetime:tt>),*]) => {
        pub trait $visitor {
            type Item;
            $(
                paste::paste! {
                    pub fn [<visit_$type:lower>](&mut self, item: &$type) -> Self::Item;
                }
            )*
        }

        $(
            paste::paste! {
                impl<$lifetime> $type<$lifetime> {
                    pub fn accept<V>(&self, visitor: &mut V) -> V::Item
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
