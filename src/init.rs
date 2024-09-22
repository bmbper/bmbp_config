use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use std::sync::LazyLock;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "web/template"]
pub(crate) struct PageAssets;

pub(crate) static CTX_TERA: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    // 加载嵌入的模板文件
    for file in PageAssets::iter() {
        if let Some(content) = PageAssets::get(file.as_ref()) {
            let content_str = std::str::from_utf8(content.data.as_ref()).expect("读取模板内容失败");
            tera.add_raw_template(file.as_ref(), content_str)
                .expect("加载模板内容到模板引擎失败");
        }
    }
    tera
});

#[derive(RustEmbed)]
#[folder = "web/static"]
struct StaticAssets;

pub(crate) fn build_bmbp_static_router() -> Router {
    Router::with_path("bmbp/ui/config/<**path>").get(static_embed::<StaticAssets>())
}
