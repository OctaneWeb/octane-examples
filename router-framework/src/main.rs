use crate::router_builder::RouterBuilder;
use crate::server_settings::Settings;
use octane::server::Octane;
use std::error::Error;
use std::path::PathBuf;

pub mod router_builder;
pub mod server_settings;

#[octane::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Octane::new();
    // we create a settings struct to store our info which we'll serve
    let mut settings = Settings::new();
    settings
        // we add the content with location here
        .content("/", "Hello world")
        .content("/route2", "this is second route")
        .content("/route3", "this is another route")
        // load our test.html file at this url
        .file(
            "/file",
            PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/pub/test.html")),
        );
    let routes = RouterBuilder::from(settings);
    routes.build(&mut app);
    app.listen(8000).await
}
