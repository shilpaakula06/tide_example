#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, Tide!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

