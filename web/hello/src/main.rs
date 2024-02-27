use tide::Request;

async fn hello(_req: Request<()>) -> tide::Result {
    Ok("Hello, World!".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/hello").get(hello);
    app.listen("127.0.0.1:3030").await?;
    Ok(())
}
