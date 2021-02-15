use std::{fs::File, io::{Read, Write}};

use crate::compiler::lexar::*;

//boba init [create config]
//boba build (*.boba)
//boba chleeisbaka
//ㄴ True

pub fn cli(str: Vec<String>) {
    if str.len() == 1 {
        println!("Commands List:");
        println!("\tboba init - Make Config File");
        println!("\tex)\t>boba init");
        println!("\tboba build [filename].boba [filename].html - Build boba");
        println!("\tex)\t>boba build ./test.boba ./test.html");
        println!("\tboba run [filename].boba - Show Build Result");
        println!("\tex)\t>boba run ./test.boba");
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
}"#.as_bytes());
            println!("Config done.")
        },
        "build" => {
            //cargo run build ./html.boba ./html.html            
            let mut file = File::open(str[2].to_string()).expect("메세지 부분에 오류 남");
            let mut boba = String::new();
            let _ = file.read_to_string(&mut boba);
            let mut file = File::create(str[3].clone()).expect("파일 생성에 문제 있음");
            let _ = file.write_all(Lexar::new(boba).as_bytes());
            println!("Done.")
        },
        "run" => {
            //cargo run run ./html.boba
            let mut file = File::open(str[2].to_string()).expect("메세지 부분에 오류 남");
            let mut boba = String::new();
            let _ = file.read_to_string(&mut boba);
            println!("{}", Lexar::new(boba));
        },
        "help" | _ => {
            println!("Commands List:");
            println!("\tboba init - Make Config File");
            println!("\tex)\t>boba init");
            println!("\tboba build [filename].boba [filename].html - Build boba");
            println!("\tex)\t>boba build ./test.boba ./test.html");
            println!("\tboba run [filename].boba - Show Build Result");
            println!("\tex)\t>boba run ./test.boba");
            println!("\tboba help | [Somethings] - Show Commands List");
            println!("\tex)\t>boba help");
        },
    }
}