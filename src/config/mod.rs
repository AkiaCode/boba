use std::{fs::File, io::Read};

pub fn config(key: &str) -> String {
    let mut file = File::open(".boba.json").expect("warning: not found config file");
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    let result = json::parse(buf.as_mut_str()).unwrap();
    if result[key].is_empty() {
        eprintln!("Error: Config Not Found");
    }
    return result[key].to_string();
}
