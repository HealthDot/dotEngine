# Rust Starter Project

A basic git repository containing some bare essentials
in order to quickly start a new rust project

Projects start with a lib named "lib" from src/lib.rs
and a bin named "bin" from src/main.rs

## How to use with cargo-start plugin
```shell
# Install the cargo-start plugin
$ cargo install --git https://github.com/masmullin2000/cargo-start

# Use the cargo start plugin
$ cargo start my_brand_new_project
```

## How To Use Directly
```shell
$ git clone --depth 1 https://github.com/masmullin2000/rust_starter_project <my_project_name>
$ rm -rf <my_project_name>/.git
$ cd <my_project_name>
$ git init
$ sed -i "s/rust_starter_project/<my_project_name>/g" Cargo.toml
```

Where <my_project_name> is the name of the project you want to start