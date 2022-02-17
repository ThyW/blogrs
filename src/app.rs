use rocket::fs::{FileServer, Options};
use rocket_dyn_templates::Template;

use crate::blog::BlogHandler;
use crate::commands;
use crate::error::BlogrsResult;
use crate::index::IndexHandler;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct AppConfig {
    home_page: Option<String>,
    blog_dir: Option<String>,
    home_route: Option<String>,
    blog_route: Option<String>,
    blog_index: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            home_page: Some("index.html".to_string()),
            blog_dir: Some("blogs/".to_string()),
            home_route: Some("/".to_string()),
            blog_route: Some("/blogs".to_string()),
            blog_index: Some("blog_index".to_string()),
        }
    }
}

impl AppConfig {
    pub fn from_commands(input: Vec<commands::Command>) -> Self {
        let mut s = Self::default();

        for command in input {
            match command {
                commands::Command::BlogDir(blog_dir) => s.blog_dir = Some(blog_dir),
                commands::Command::HomePage(hp) => s.home_page = Some(hp),
                commands::Command::HomeRoute(hr) => s.home_route = Some(hr),
                commands::Command::BlogRoute(br) => s.blog_route = Some(br),
                commands::Command::BlogIndex(br) => s.blog_route = Some(br),

                _ => (),
            }
        }

        s
    }
}

#[derive(Debug)]
pub struct App {
    config: AppConfig,
}

impl App {
    #![allow(dead_code)]
    fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    pub fn with_config(c: AppConfig) -> Self {
        Self { config: c }
    }

    pub fn build(&mut self) -> Self {
        unimplemented!()
    }

    pub async fn run(&self) -> BlogrsResult {
        let mut rocket = rocket::build();
        let cfg = self.config.clone();
        if cfg.home_route.is_some() && cfg.home_page.is_some() {
            let (page, route) = (cfg.home_page.unwrap(), cfg.home_route.unwrap());
            let ih = IndexHandler::new(page, route.clone());
            rocket = rocket.mount(route, ih);
        }

        if cfg.blog_dir.is_some() && cfg.blog_route.is_some() && cfg.blog_index.is_some() {
            let (route, dir, index) = (
                cfg.blog_route.unwrap(),
                cfg.blog_dir.unwrap(),
                cfg.blog_index.unwrap(),
            );
            let index = Cow::from(index);
            let bh = BlogHandler::new(dir.clone(), index);
            rocket = rocket.mount(route.clone(), bh);

            rocket = rocket.mount(route, FileServer::new(dir, Options::default()))
        }

        rocket = rocket.attach(Template::fairing());

        rocket.launch().await?;

        Ok(())
    }
}
