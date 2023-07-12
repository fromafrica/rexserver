use rex::Http;
use rex::start;

mod rex;

#[tokio::main]
async fn main() {

    // setup server
    let mut app = Http::new();

    // define routes
    app.get("/index.html", "/dist/index.html");
    app.get("/favicon.ico", "/dist/favicon.ico");
    app.get("/test", "/dist/404.html");

    let routes = app.get_routes;

    // handle requests
    start(routes).await
}