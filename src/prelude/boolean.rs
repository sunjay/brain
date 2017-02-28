use operations::scope::ScopeStack;

pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    // boolean and operator (operator&&) and boolean or operator (operator||)
    // These operations have special names because they are not regular functions
    // that can be defined or overloaded
    // The reason these are not definable is because they have to support short
    // circuiting. This behaviour cannot be modelled by a trait, so these special
    // operators are not definable by the user.
    //TODO
}
