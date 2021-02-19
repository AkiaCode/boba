use boba::cli::cli;
use std::env;

fn main() {
    let mut env = Vec::new();
    for i in env::args() {
        env.push(i);
    }
    cli(env);
}
