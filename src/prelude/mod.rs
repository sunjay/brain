use core::primitives::*;
use core::io::*;

use operations::scope::ScopeStack;

/// Populates the given scope with all declarations that
/// should be available in every module at the top level
/// (known as the Prelude)
pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    array::define_array(scope);

    let bool_type = boolean::define_boolean(scope);
    let u8_type = u8::define_u8(scope, bool_type);

    stdio::define_stdin(scope, u8_type);
    stdio::define_stdout(scope);
}
