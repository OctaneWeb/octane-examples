use octane::prelude::*;
use std::error::Error;

#[octane::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Octane::new();
    app.with_router(routes()?);
    app.listen(8000, || {
        println!("{:?}", "Server has started!!");
    })
    .await
}

pub fn routes() -> Result<Router, Box<dyn Error>> {
    let mut routes = Router::new();
    routes.get(
        "/",
        route!(|req, res| {
            res.send(r#"My Website!, go to second page <a href="/home">home</a> "#);
            Flow::Next
        }),
    )?;
    routes.get(
        "/home",
        route_next!(|req, res| {
            res.send(r#"You are at second page"#);
        }),
    )?;
    Ok(routes)
}
