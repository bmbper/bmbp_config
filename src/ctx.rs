use bmbp_ctx_vars::{ContexVars, BMBP_CONTEXT_VARS};
use tera::Context;

pub(crate) fn ctx_init() -> Context {
    let mut ctx = tera::Context::new();
    ctx.insert(
        ContexVars::AppCode.to_string(),
        &(*BMBP_CONTEXT_VARS).get_value::<ContexVars>(ContexVars::AppCode),
    );
    ctx.insert(
        ContexVars::AppName.to_string(),
        &(*BMBP_CONTEXT_VARS).get_value::<ContexVars>(ContexVars::AppName),
    );
    ctx.insert(
        ContexVars::AppNavName.to_string(),
        &(*BMBP_CONTEXT_VARS).get_value::<ContexVars>(ContexVars::AppNavName),
    );
    ctx.insert(
        ContexVars::AppLoginName.to_string(),
        &(*BMBP_CONTEXT_VARS).get_value::<ContexVars>(ContexVars::AppLoginName),
    );
    ctx.insert(
        ContexVars::AppCopyRight.to_string(),
        &(*BMBP_CONTEXT_VARS).get_value::<ContexVars>(ContexVars::AppCopyRight),
    );
    ctx
}
