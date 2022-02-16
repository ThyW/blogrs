I'm still WIP.

# About
`blogrs` is a command line tool and a web server for quickly being able to deploy your blogs to the internet. It is written in Rust, hence the speed and safety. It offers a moderate amount of configurability and flexibility.

# Installation
1. Have Rust installed 
2. Clone this repository
3. Build it by running:

```console
cargo build --release
```

You can then optionally link the executable to somewhere in your path.

# Quickstart
You need: an `index.html`(default name, can be whatever you want) file, a `Rocket.toml` file for configuring the web server(you can use the default file from this repository), a `blog_index.html.tera` file for quick blog preview and finally, a directory containing your individual blogs in `html` format.

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
