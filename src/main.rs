#[tokio::main]
async fn main() {
    let mut http = under::http();
    http.at("/@internal/status")
        .get(under::endpoints::simple(|| under::Response::text("ok")));
    println!("listening on 0.0.0.0:8080...");
    http.listen("0.0.0.0:8080").await.unwrap();
}
