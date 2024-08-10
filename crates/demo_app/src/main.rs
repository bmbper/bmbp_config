use bmbp_lib_ui::build_bmbp_ui_lib_router;
use bmbp_config::build_bmbp_config_router;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let host = "0.0.0.0:9002";
    tracing::info!("启动初始化服务,监听地址:{}......", host);
    let acceptor = TcpListener::new(host).bind().await;
    let mut router = Router::new();
    router = router.push(build_bmbp_ui_lib_router());
    router = router.push(build_bmbp_config_router());
    Server::new(acceptor).serve(router).await;
}
