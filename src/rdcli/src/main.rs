use tide::Request;

struct 

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/webhook").post(webhook_endpoint);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn webhook_endpoint(mut req: Request<()>) -> tide::Result {
    let body: serde_json::Value = req.body_json().await?;

    Ok("OK".into())
}