use super::token::Token;
pub struct Lexar;

/**
 *  1, [html] '.' 2, [lang="en-US" ',' ....]
 *  1, head | 1, '.'head
 *  1, title 2, ["asdsadasdsdasd"] 3, '.'title
 */

impl Lexar {
    pub fn new(string: String) -> String {
        let mut tokens= vec![];
        for i in string.lines() {
            if i.is_empty() { continue; }
            let mut tag = i.split(".").nth(0).unwrap(); // 0. [html] | 1. [lang="asdasd",dasd="asdasd"]

            if tag.contains("\"") { 
                // 'p' "This is mypage" .p
                if !i.split("\"").nth(0).unwrap().is_empty() {
                    tokens.push(Token { keyword: String::from("TAG"), value: i.split("\"").nth(0).unwrap().trim().to_string() });
                    tokens.push(Token { keyword: String::from("MSG"), value: i.split("\"").nth(1).unwrap().trim().to_string() });
                    //"[This is mypage]"
                }
                tag = i.trim().split("\"").nth(2).unwrap_or("None"); // "." e.g p "This is mypage" [.p]
                if tag == "None" {
                    continue;
                }
            } else if tag.trim().is_empty() { 
                tag = i;
            }
            if !tag.is_empty() {
                tokens.push(Token { keyword: String::from("TAG"), value: tag.trim().to_string() });
                let data = i.trim().split(".").nth(1).unwrap_or("None");
                if data != "None" { // html.[lang="en-US",asda="asd"]
                    tokens.push(Token { keyword: String::from("DATA"), value: data.trim().to_string() });
                }
            }
        }
        return parse(tokens);
    }
}

pub fn parse(tokens: Vec<Token>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>");
    for (n, i) in tokens.iter().enumerate() {
        if i.value.is_empty() { continue; }
        if i.keyword == "TAG" {
            html.push_str( &("<".to_string() + &i.value.to_string()));

            if tokens.len()-1 >= n+1 {
                if tokens[n+1].keyword == "DATA" {
                    let mut a = String::new();
                    for i in tokens[n+1].value.split(",") {
                        a.push_str(&(i.to_owned() + " "));
                    }
                    a.remove(a.len()-1);
                    html.push_str(&(" ".to_string() + &a.to_owned() + ">"));
                } else {
                    html.push_str(">");
                }
            }
        } else if i.keyword == "MSG" {
            html.push_str(&i.value);
        }
    }
    html.push_str(">");
    return html;
}