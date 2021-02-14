use std::env;
use boba::cli::cli;

fn main() {
    let mut env = Vec::new();
    for i in env::args() { env.push(i); }
    cli(env);
}
