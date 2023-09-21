use std::collections::HashMap;

use crate::token::TokenType;

pub fn get_keywords() -> Box<HashMap<String, TokenType>> {
    let mut keywords = Box::new(HashMap::new());

    keywords.insert("and".to_string(), TokenType::AND);
    keywords.insert("class".to_string(), TokenType::CLASS);
    keywords.insert("else".to_string(), TokenType::ELSE);
    keywords.insert("false".to_string(), TokenType::FALSE);
    keywords.insert("for".to_string(), TokenType::FOR);
    keywords.insert("fun".to_string(), TokenType::FUN);
    keywords.insert("if".to_string(), TokenType::IF);
    keywords.insert("nil".to_string(), TokenType::NIL);
    keywords.insert("or".to_string(), TokenType::OR);
    keywords.insert("print".to_string(), TokenType::PRINT);
    keywords.insert("return".to_string(), TokenType::RETURN);
    keywords.insert("super".to_string(), TokenType::SUPER);
    keywords.insert("this".to_string(), TokenType::THIS);
    keywords.insert("true".to_string(), TokenType::TRUE);
    keywords.insert("var".to_string(), TokenType::VAR);
    keywords.insert("while".to_string(), TokenType::WHILE);

    return keywords;
}
