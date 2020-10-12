#[tokio::main]
async fn main() {
    let mut stack = short::Stack::new();
    stack
        .at("/@internal/status")
        .get(short::endpoint::static_endpoint(|| {
            short::Response::text("ok")
        }));
    stack.listen("0.0.0.0:8080").await.unwrap();
}
