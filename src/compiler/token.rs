#[derive(Debug)]
pub struct Token {
    pub keyword: String, //[html].lang="en-US", None, [[.]html]
    pub value: String,   //html.[lang="en-US"],[asd="sdasd"], [everything]
}

impl Token {
    pub fn new(keyword: String, value: String) -> Token {
        Token { keyword, value }
    }
}
