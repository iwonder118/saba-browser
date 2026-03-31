use core::str;

use alloc::string::String;
use alloc::vec::Vec;

use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    reconsume: bool,    
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attribute: Vec<Attribute>
    },
    
    EndTag {
        tag: String,
    },
    
    Char(char),
    Eof,
}
