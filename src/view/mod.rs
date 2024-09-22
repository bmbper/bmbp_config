use crate::ctx::ctx_init;
use crate::init::CTX_TERA;
use salvo::prelude::Text;
use salvo::{handler, Request, Response};

#[handler]
pub async fn dict_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    resp.render(Text::Html((*CTX_TERA).render("dict.html", &ctx).unwrap()));
}

#[handler]
pub async fn vars_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    resp.render(Text::Html((*CTX_TERA).render("vars.html", &ctx).unwrap()));
}

#[handler]
pub async fn security_view(_: &mut Request, res: &mut Response) {
    let ctx = ctx_init();
    res.render(Text::Html(
        (*CTX_TERA).render("security.html", &ctx).unwrap(),
    ));
}

#[handler]
pub async fn init_view(_: &mut Request, resp: &mut Response) {
    let ctx = ctx_init();
    resp.render(Text::Html((*CTX_TERA).render("init.html", &ctx).unwrap()));
}
