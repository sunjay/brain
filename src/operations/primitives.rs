//! Internal struct used by ScopeStack to provide read-only access to primitive type TypeIds
//!

//! The point of having this struct is so that accessing primitives can be type checked and any
//! problems with that can be caught at compile time. Unfortunately, registering primitives is not
//! as easy to make type check, so we get by with runtime checks instead. The reasoning behind
//! accepting this trade-off is that registration happens early and in a consistent way that is
//! easy to test and catch whereas access can happen anytime and is not necessarily testable.

use super::scope::TypeId;

macro_rules! impl_primitives {
    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        pub struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            pub fn new() -> Primitives {
                $name {
                    $($field_name: None,)*
                }
            }

            pub fn register(&mut self, name: &str, type_id: TypeId) {
                match name {
                    $(
                        stringify!($field_name) => {
                            // Only allowed to define primitive types once
                            // This helps catch bugs where we are accidentally defining special
                            // primitives twice for some reason
                            debug_assert!(self.$field_name.is_none(), format!("Redefined `{}` primitive in scope", stringify!($field_name)));

                            self.check_collision(type_id);

                            self.$field_name = Some(type_id)
                        },
                    )*
                    _ => panic!(format!("Attempt to register unknown primitive: `{}`", name)),
                }
            }

            /// Avoids collisions between values stored in the properties of a struct
            /// All values must have the same type for this to work
            fn check_collision(&self, type_id: TypeId) {
                $(
                    if self.$field_name == Some(type_id) {
                        panic!(format!("TypeId `{}` is already registered to another primitive: `{}`", type_id, stringify!($field_name)));
                    }
                )*
            }

            $(
                pub fn $field_name(&self) -> TypeId {
                    self.$field_name.expect(format!("Expected a TypeId to be defined for the primitive `{}`", stringify!($field_name)).as_str())
                }
            )*
        }
    }
}

/// To add a primitive, simply add a field to the following struct
/// Everything else will be generated for you
impl_primitives! {
    pub struct Primitives {
        unit: Option<TypeId>,
        array: Option<TypeId>,
        bool: Option<TypeId>,
        u8: Option<TypeId>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_define_primitives() {
        let mut prim = Primitives::new();

        // Intentionally not mimicking actual code (unit is 0 in actual code)
        prim.register("unit", 22);
        prim.register("array", 829);
        prim.register("bool", 193);
        prim.register("u8", 4);

        assert_eq!(prim.unit(), 22);
        assert_eq!(prim.array(), 829);
        assert_eq!(prim.bool(), 193);
        assert_eq!(prim.u8(), 4);
    }

    #[test]
    #[should_panic(expected = "Expected a TypeId to be defined for the primitive `unit`")]
    fn access_without_registration() {
        let prim = Primitives::new();

        prim.unit();
    }

    #[test]
    #[should_panic(expected = "Redefined `unit` primitive in scope")]
    fn redefined_primitive_same_type_id() {
        let mut prim = Primitives::new();

        prim.register("unit", 0);
        prim.register("bool", 2);
        prim.register("unit", 0);
    }

    #[test]
    #[should_panic(expected = "Redefined `bool` primitive in scope")]
    fn redefined_primitive() {
        let mut prim = Primitives::new();

        prim.register("bool", 0);
        prim.register("array", 6);
        prim.register("bool", 2);
    }

    #[test]
    #[should_panic(expected = "TypeId `0` is already registered to another primitive: `unit`")]
    fn type_collision() {
        let mut prim = Primitives::new();

        prim.register("unit", 0);
        prim.register("array", 0);
    }

    #[test]
    #[should_panic(expected = "Attempt to register unknown primitive: `foo`")]
    fn unknown_primitive() {
        let mut prim = Primitives::new();

        prim.register("foo", 0);
    }
}
