mod io;
mod boolean;
mod array;
mod u8;

use operations::scope::ScopeStack;

/// Populates the given scope with all declarations that
/// should be available in every module at the top level
pub fn populate_scope(scope: &mut ScopeStack) {
    // Taking advantage of the scope system to simulate modules
    // This will be replaced with something better in:
    // https://github.com/brain-lang/brain/issues/37
    scope.push_scope();

    self::io::populate_scope(scope);
    self::array::populate_scope(scope);
    self::u8::populate_scope(scope);
}
