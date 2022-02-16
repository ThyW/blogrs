use rocket::fs::NamedFile;
use rocket::http::Method;
use rocket::route::{Handler, Outcome, Route};
use rocket::{Data, Request};

#[derive(Clone)]
pub struct IndexHandler {
    page: String,
    route: String,
}

#[rocket::async_trait]
impl Handler for IndexHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _data: Data<'r>) -> Outcome<'r> {
        let file = NamedFile::open(&self.page).await;
        return Outcome::from(req, file);
    }
}

impl Into<Vec<Route>> for IndexHandler {
    fn into(self) -> Vec<Route> {
        let r = self.route.clone();
        vec![Route::new(Method::Get, &r, self)]
    }
}

impl IndexHandler {
    pub fn new(page: String, route: String) -> Self {
        Self { page, route }
    }
}
