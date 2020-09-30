use crate::server_settings::Content;
use crate::server_settings::Settings;
use octane::route;
use octane::router::Closure;
use octane::router::{Flow, Route};
use octane::server::Octane;
use std::convert::From;

pub struct RouterBuilder {
    settings: Settings,
}

impl From<Settings> for RouterBuilder {
    fn from(settings: Settings) -> Self {
        RouterBuilder { settings }
    }
}

impl RouterBuilder {
    pub fn build(self, app: &mut Octane) {
        for (loc, content) in self.settings.pages.into_iter() {
            app.get(&loc, Self::get_route(content)).unwrap();
        }
    }
    pub fn get_route(content: Content) -> Closure {
        route!(|req, res| {
            if content.is_file() {
                res.send_file(&content.file.as_ref().unwrap())
                    .expect("file not found");
            } else {
                res.send(content.raw_content.as_ref().unwrap());
            }
            Flow::Next
        })
    }
}
