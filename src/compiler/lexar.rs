pub struct Lexar;

/**
 *  1, [html] '.' 2, [lang="en-US" ',' ....]
 *  1, head | 1, '.'head
 *  1, title 2, ["asdsadasdsdasd"] 3, '.'title
 */

impl Lexar {
    pub fn new(string: String) {
        let mut tokens= vec![];
        for i in string.lines() {
            if i.is_empty() { continue; }
            let mut tag = i.split(".").nth(0).unwrap(); // 0. [html] | 1. [lang="asdasd",dasd="asdasd"]
            println!("{}", i);
            if tag.contains("\"") { 
                // 'p' "This is mypage" .p
                if !i.split("\"").nth(0).unwrap().is_empty() {
                    tokens.push(i.split("\"").nth(0).unwrap().trim());
                }
                // . 찾기
                tag = i.trim().split("\"").nth(2).unwrap_or("None"); // "." e.g p "This is mypage" '.p'
                if tag == "None" {
                    continue;
                }
            } else if tag.trim().is_empty() { 
                tag = i;
            }
            tokens.push(tag.trim());
        }
        println!("{:?}", tokens);
    }
}

pub fn parse(string: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>");

    return html;
}