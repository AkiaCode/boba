use std::{
    fs::File,
    io::{Read, Write},
};

use crate::compiler::lexar::*;
use crate::config::config;

pub fn cli(str: Vec<String>) {
    if str.len() == 1 {
        println!("Commands List:");
        println!("\tboba init - Make Config File");
        println!("\tex)\t>boba init");
        println!("\tboba build [filename].boba [filename].html - Build boba");
        println!("\tex)\t>boba build ./test.boba ./test.html");
        println!("\tboba run [filename].boba - Show Build Result");
        println!("\tex)\t>boba run ./test.boba");
        println!("\tboba config [key] - Show Config");
        println!("\tex)\t>boba config");
        println!("\tboba help | [Somethings] - Show Commands List");
        println!("\tex)\t>boba help");
        return;
    }

    match str[1].as_str() {
        "init" => {
            let mut file = File::create(".boba.json").expect("파일 생성에 문제 있음");
            let _ = file.write_all(
                r#"{
    "outDir": "",
    "inDir": ""
}"#
                .as_bytes(),
            );
            println!("Config done.")
        }
        "build" => {
            //cargo run build ./html.boba ./html.html
            let mut file =
                File::open(config("inDir") + &str[2].to_string()).expect("메세지 부분에 오류 남");
            let mut boba = String::new();
            let _ = file.read_to_string(&mut boba);
            let mut file = File::create(config("outDir") + &str[3]).expect("파일 생성에 문제 있음");
            let _ = file.write_all(Lexar::new(boba).as_bytes());
            println!("Done.")
        }
        "run" => {
            //cargo run run ./html.boba
            let mut file =
                File::open(config("inDir") + &str[2].to_string()).expect("메세지 부분에 오류 남");
            let mut boba = String::new();
            let _ = file.read_to_string(&mut boba);
            println!("{}", Lexar::new(boba));
        }
        "config" => {
            if str.len() == 3 {
                println!("{}", config(&str[2]));
            } else {
                println!(
                    "Configs:\ninDir: {}\noutDir: {}",
                    config("inDir"),
                    config("outDir")
                )
            }
        }
        "help" | _ => {
            if str[1] != "help" {
                eprintln!("Error: Command Not Found\n");
            }
            println!("Commands List:");
            println!("\tboba init - Make Config File");
            println!("\tex)\t>boba init");
            println!("\tboba build [filename].boba [filename].html - Build boba");
            println!("\tex)\t>boba build ./test.boba ./test.html");
            println!("\tboba run [filename].boba - Show Build Result");
            println!("\tex)\t>boba run ./test.boba");
            println!("\tboba config [key] - Show Config");
            println!("\tex)\t>boba config");
            println!("\tboba help | [Somethings] - Show Commands List");
            println!("\tex)\t>boba help");
        }
    }
}
