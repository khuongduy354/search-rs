use std::{
    collections::HashMap,
    fs::{self, File},
    vec,
};
use xml::{reader::XmlEvent, EventReader};

// parse string into array of words (called tokens)
#[derive(Debug)]
struct Lexer<'a> {
    source: &'a [char],
}
impl<'a> Lexer<'a> {
    fn new(source: &'a [char]) -> Self {
        Self { source }
    }

    //pop 0..n of source,
    fn chop_left(&mut self, n: usize) -> &'a [char] {
        let result = &self.source[0..n];
        self.source = &self.source[n..];
        result
    }

    //move index while predicate holds
    fn move_idx_while(&mut self, predicate: fn(&char) -> bool) -> usize {
        let mut n = 0;
        while n < self.source.len() && predicate(&self.source[n]) {
            n += 1;
        }
        n
    }
    fn trim_white_space(&mut self) {
        if self.source.len() == 0 {
            return;
        }
        while self.source[0].is_whitespace() {
            if self.source.len() > 1 {
                self.source = &self.source[1..];
            } else {
                self.source = &[];
                return;
            }
        }
    }

    fn parse_next_token(&mut self) -> Option<&'a [char]> {
        self.trim_white_space();
        if self.source.len() == 0 {
            return None;
        }
        if self.source[0].is_alphabetic() {
            let end_pos = self.move_idx_while(|x| x.is_alphabetic());
            Some(self.chop_left(end_pos))
        } else if self.source[0].is_numeric() {
            let end_pos = self.move_idx_while(|x| x.is_numeric());
            Some(self.chop_left(end_pos))
        } else {
            Some(self.chop_left(1))
        }
    }
}

// fn parse_to_hash_map(source: String) {
//     let map: HashMap<String, i32> = HashMap::new();
// }

fn read_all_xml(file_path: &str) -> Result<String, std::io::Error> {
    let mut result = String::from("");
    let file = File::open(file_path)?;
    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::Characters(s)) => {
                result.push_str(&s);
            }
            _ => {}
        }
    }
    Ok(result)
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_token()
    }
}
fn main() {
    let file_path = "docs.gl/gl4/glActiveTexture.xhtml";
    let content = read_all_xml(file_path)
        .expect("cant read file")
        .chars()
        .collect::<Vec<_>>();
    // println!("{}", content.iter().collect::<String>());
    let parser = Lexer::new(&content);
    for token in parser {
        println!("{}", token.iter().collect::<String>());
    }
}
