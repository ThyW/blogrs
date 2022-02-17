use crate::error::BlogrsResult;
use rocket::http::{Method, Status};
use rocket::route::{Handler, Outcome, Route};
use rocket::{Data, Request};
use rocket_dyn_templates::Template;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct BlogHandler {
    dir: String,
    index: Cow<'static, str>,
}

impl BlogHandler {
    pub fn new(dir: String, index: Cow<'static, str>) -> Self {
        Self { dir, index }
    }
    fn inner(&self) -> BlogrsResult<HashMap<&str, Vec<(String, String)>>> {
        let mut hm = HashMap::new();
        let mut v = Vec::new();
        for e in fs::read_dir(&self.dir)? {
            let entry = e?;
            if entry.file_name().to_string_lossy().ends_with(".html") {
                if let Some(name) = entry.file_name().to_string_lossy().strip_suffix(".html") {
                    let name = name.replace("_", " ");
                    v.push((entry.file_name().to_string_lossy().to_string(), name))
                }
            }
        }

        hm.insert("entries", v);

        Ok(hm)
    }
}

#[rocket::async_trait]
impl Handler for BlogHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _data: Data<'r>) -> Outcome<'r> {
        let res = self.inner();
        if res.is_ok() {
            return Outcome::from(req, Template::render(self.index.clone(), &res.unwrap()));
        }

        Outcome::Failure(Status::from_code(500).unwrap_or(Status::new(500)))
    }
}

impl Into<Vec<Route>> for BlogHandler {
    fn into(self) -> Vec<Route> {
        // let r = self.route.clone();
        vec![Route::new(Method::Get, "/", self)]
    }
}
