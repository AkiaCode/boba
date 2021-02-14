use std::{fs::File, io::{Read, Write}};

use crate::compiler::lexar::*;

//boba init [create config]
//boba build (*.boba)
//boba chleeisbaka
//ㄴ True

pub fn cli(str: Vec<String>) {
    match str[1].as_str() {
        "init" => {
            let mut file = File::create(".boba.json").expect("파일 생성에 문제 있음");
            let _ = file.write_all(
r#"{
    "outDir": "" 
}"#.as_bytes());
            println!("Config done.")
        },
        "build" => (),
        "run" => {
            //cargo run run ./html.boba
            let mut file = File::open(str[2].to_string()).expect("메세지 부분에 오류 남");
            let mut boba = String::new();
            let _ = file.read_to_string(&mut boba);
            Lexar::new(boba);
        }
        _ => (),
    }
}