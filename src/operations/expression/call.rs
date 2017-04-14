use parser::{Expression, CallArgs, Identifier};
use memory::MemoryBlock;

use operations::{Error, OperationsResult};
use operations::item_type::{ItemType, FuncArgType};
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
            let (target, method_name) = resolve_field_name(scope, *target, field)?;
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
    // The first stage of calling a function is finding an implementation that matches the correct
    // function signature.
    let method_args_types = args.iter().map(|arg| match *arg {
        //TODO: Update this when more numeric types are added
        ScopeItem::NumericLiteral(..) => FuncArgType::Arg(scope.primitives().u8()),
        ScopeItem::ByteLiteral(..) => FuncArgType::Array {item: scope.primitives().u8(), size: None},
        ScopeItem::Array {item, ..} => FuncArgType::Array {item: item, size: None},
        ref arg => FuncArgType::Arg(arg.type_id()),
    }).collect();

    // TODO: Since we don't have proper generics, we just search through and try every function
    // with the given name to see if its arguments match. This is more similar to what C++ does
    // than Rust, but it works for the timebeing.
    // We keep searching until we find something that matches or we return the first error.
    scope.lookup(&method_name).into_iter().fold(Err(Error::UnresolvedName(method_name.clone())), |acc, item| acc.or_else(|err| match *item {
        ScopeItem::BuiltInFunction {type_id, ref operations} => {
            if scope.get_type(type_id).matches_signature(&method_args_types, target_type) {
                Ok(operations.clone())
            }
            else {
                Err(err)
            }
        },
        _ => Err(err),
    })).and_then(|operations| {
        unimplemented!();
    })
}

/// Returns the full path of the target type with the field appended to it
/// e.g. If target's type is `std::Foo` and field is `bar`, you get: `std::Foo::bar`
/// Also returns the target ScopeItem of this operation
/// Since this is a field access, it needs to operate on the target object that the `target`
/// expression refers to
fn resolve_field_name(scope: &ScopeStack, target: Expression, field: Identifier) -> Result<(ScopeItem, Identifier), Error> {
    // The full path of the target type
    // Something like `std::foo::Foo` or `u8` or `()`
    let (target_instance, target_type_path) = match target {
        Expression::Identifier(target_name) => scope.lookup(&target_name).first().ok_or_else(|| {
            Error::UnresolvedName(target_name.clone())
        }).and_then(|item| match **item {
            ScopeItem::Constant {type_id, ..} => Ok(((*item).clone(), type_id)),
            ScopeItem::TypedBlock {type_id, ..} => Ok(((*item).clone(), type_id)),
            ScopeItem::Array {..} => unimplemented!(),
            ScopeItem::BuiltInFunction {type_id, ..} => Err(Error::UnresolvedField {
                target_type: scope.get_type(type_id).clone(),
                field: field.clone(),
            }),
            // These are unreachable because numeric literals and byte literals are never stored
            // directly
            ScopeItem::NumericLiteral(..) | ScopeItem::ByteLiteral(..) => unreachable!(),
        }).map(|(item, type_id)| (item, scope.get_type_name(type_id).clone())),

        //TODO: It's likely that the below line will just not work at all
        // because the scope item returned in the tuple is not the target we should return.
        // I think we need to map on this result and return the field as the target...not sure.
        //Expression::Access {target, field} => resolve_field_name(scope, *target, field),
        Expression::Access {target, field} => unimplemented!(),

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
    }?;

    Ok((target_instance, target_type_path.concat(field)))
}
