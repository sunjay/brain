use parser::{Expression, CallArgs, Identifier};
use memory::MemoryBlock;

use operations::{Error, OperationsResult};
use operations::scope::{TypeId, ScopeStack, ScopeItem, FuncArgs};

/// Evaluates the arguments first, then supplies them to the given method
pub fn call_with_exprs(
    scope: &mut ScopeStack,
    method: Expression,
    arg_exprs: CallArgs,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    let (mut args, ops): (Vec<_>, Vec<_>) = arg_exprs.into_iter().map(|expr| match expr {
        Expression::ByteLiteral(bytes) => Ok((ScopeItem::ByteLiteral(bytes), Vec::new())),
        Expression::Number(bytes) => Ok((ScopeItem::NumericLiteral(bytes), Vec::new())),
        Expression::Identifier(name) => scope.lookup(&name).first().ok_or_else(|| {
            Error::UnresolvedName(name.clone())
        }).map(|item| ((**item).clone(), Vec::new())),
        Expression::Call {..} => unimplemented!(),
        Expression::Access {..} => unimplemented!(),
        Expression::Branch {..} => unimplemented!(),
    }).collect::<Result<Vec<_>, _>>()?.into_iter().unzip();

    let (target_instance, method_name): (Option<ScopeItem>, Identifier) = match method {
        Expression::Identifier(name) => (None, name),
        Expression::Access {target, field} => {
            let (target, method_name) = resolve_field_name(scope, *target, *field)?;
            (Some(target), method_name)
        },
        // The grammar should prevent any other expressions from
        // ending up here
        //TODO: Instead of using unreachable here, maybe the type of method should not be expression
        _ => unreachable!(),
    };

    // If the method operates on some type, the instance of that type is the first argument
    if let Some(target_instance) = target_instance {
        args.insert(0, target_instance);
    }

    Ok(ops.into_iter().flat_map(|o| o.into_iter()).chain(
        call(scope, method_name, args, target_type, target)?
    ).collect())
}

/// Call the provided method with the given arguments
pub fn call(
    scope: &mut ScopeStack,
    method_name: Identifier,
    args: FuncArgs,
    target_type: TypeId,
    target: MemoryBlock,
) -> OperationsResult {
    let function = resolve_method(scope, method_name);

    unimplemented!();
}

/// Returns the full path of the target type with the field appended to it
/// e.g. If target's type is `std::Foo` and field is `bar`, you get: `std::Foo::bar`
/// Also returns the target ScopeItem of this operation
/// Since this is a field access, it needs to operate on the target object that the `target`
/// expression refers to
fn resolve_field_name(scope: &ScopeStack, target: Expression, field: Expression) -> Result<(ScopeItem, Identifier), Error> {
    // The full path of the target type
    // Something like `std::foo::Foo` or `u8` or `()`
    let target_type_path = match target {
        Expression::Identifier(target_name) => {
            unimplemented!();
        },

        Expression::Access {target, field} => resolve_field_name(scope, *target, *field),

        //TODO: ByteLiterals are valid targets for field access
        // In this case, we need to return [u8; N] where N is the length of the byte literal
        // We also likely need to allocate and store this into a temporary variable
        Expression::ByteLiteral(..) => unimplemented!(),

        //TODO: Numbers are valid targets for field access
        // In this case, we need to return u8 or another valid type
        // We also likely need to allocate and store this into a temporary variable
        Expression::Number(..) => unimplemented!(),

        //TODO: Calls are valid targets for field access
        // In this case, we need to return the type name of the return type
        // Implementing this will be a bit more complicated since we need to do the call and return
        // the ops necessary for that
        Expression::Call {method, args} => unimplemented!(),
        Expression::Branch {..} => unreachable!(),
    };

    unimplemented!();
}

fn resolve_method(scope: &ScopeStack, method_name: Identifier) {
    unimplemented!();
}
