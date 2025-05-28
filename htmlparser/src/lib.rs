use std::{collections::binary_heap::Iter, iter::Enumerate};

pub struct Element {
    pub type_: ElementType,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Element>,
    pub text: String,
}

pub enum ElementType {
    p,
    div,
    span,
    h1,
    a,
    // Add other HTML element types as needed
}

impl Element {
    pub fn new(type_: ElementType, attributes: Vec<(String, String)>, text: String) -> Self {
        Element { type_, attributes, children: Vec::new(), text }
    }
}

pub struct WebPage {
    elements: Vec<Element>,
}

// Span of a specific tag, with the starting index in the string and the ending index
struct TagSpan{
    start: usize,
    end: usize,
}

enum ParseError{
    MultipleStarts((usize, usize)),
    MultipleEnds(usize)
}

impl TagSpan{
    fn from_rustml_to_spans(rustml: &str) -> Result<Vec<Self>, ParseError>{
        let chars = rustml.chars().collect::<Vec<char>>();
        let mut spans: Vec<TagSpan> = Vec::new();
        let (mut start, mut end) = (0, 0);
        let mut is_capturing = false;
        for index in 0..chars.len(){
            match chars[index]{
                '<' => {
                    //todo
                    if is_capturing{ return Err(ParseError::MultipleStarts((start, index))) }

                    start = index;
                    is_capturing = true;

                }

                '>' => {
                    //todo
                    if !is_capturing{ return Err(ParseError::MultipleEnds((index))); }

                    end = index;
                    is_capturing = false;

                    spans.push(TagSpan { start, end });

                }

                _ => {

                }
            }
        }
        return Ok(spans);
    }
}



impl WebPage {
    pub fn from_rustml(rustml: &str) -> Result<Self, ParseError> {
        
        let elements: Vec<Element> = Vec::new();
        
        
        
        
        return WebPage { elements }
    }
}


