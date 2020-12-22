use salvo::prelude::*;
use tracing_subscriber;

#[fn_handler]
async fn hello_world(_conf: Arc<ServerConfig>, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render_plain_text("Hello World");
}
#[fn_handler]
async fn hello_world2(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render_plain_text("Hello World2");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // tracing_subscriber::fmt::init();
    let mut router = Router::new("/");
    router.get(hello_world);
    router.scope("hello2").get(hello_world2);
    let server = Server::new(router);
    server.serve().await?;
    Ok(())
}
