use octane::prelude::*;
use std::error::Error;

#[octane::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Octane::new();
    app.ssl(8080)
        .key(path!("/pub/key.pem"))
        .cert(path!("/pub/cert.pem"));
    app.get(
        "/",
        route_next!(|req, res| {
            res.send("Hello, World!");
        }),
    )?;
    app.listen(8000, || {
        println!("{:?}", "Server listening at 8000, https at 8080");
    })
    .await
}
