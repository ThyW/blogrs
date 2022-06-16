Still WIP.

# About
`blogrs` is a command line tool and a web server for quick deployment of your blogs to the internet. Written in rust, it offers a moderate amount of configurability and flexibility while also being safe and fast.

# Installation
1. Have Rust installed.
2. Clone this repository.
3. Build it by running:

```console
cargo build --release
```

You can then optionally link the executable to somewhere in your path.

# Quickstart
There are three files you need:
1. An `index.html` or alternatively a differently named html file which models the front page of your blog.
2. A `Rocket.toml` file. You can create your own based on this [guide](https://rocket.rs/v0.5-rc/guide/configuration/) or you can use the one which came with this repo.
3. A `blog_index.html.tera` file. [Tera](https://tera.netlify.app/) is a template engine which is used for dynamically generating a list of blogs that the viewers of your blog can access. Again, you can use the file in this repository or you can write your own.

You could start by placing all these files into a single directory: 

```
my_blog/
  blogs/
    1_How_to_feed_your_cat.html
    2_How_to_write_readmes.html
  blog_index.html.tera
  index.html
  Rocket.toml
```

If `Rocket.toml` is configured correctly, running `blogrs --default` should run the app and deploy your blogs to the web address specified.
