use crate::{context::Context, value::VerifierFailureAction};

#[test]
fn test_create_context() {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("test");

    let func_ret_ty = context.create_void_type();
    let func_param_tys = [];

    let func_ty = context.create_func_type(&func_ret_ty, &func_param_tys, false);
    let func = module.add_function("main", &func_ty);

    let entry = context.append_basic_block(&func, "entry");
    builder.position_at_end(&entry);

    builder.build_return_void();

    assert!(func.verify_function(VerifierFailureAction::PrintMessage));

    module.dispose();
    builder.dispose();
    context.dispose();
}
