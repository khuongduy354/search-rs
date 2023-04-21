// use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
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
    fn into_hash_map() {}

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
    fn to_hash_map(tok: String, hmap: &mut HashMap<String, i32>) {
        if let Some(_freq) = hmap.get_mut(&tok) {
            // hmap.insert(tok, _freq + 1);
            *_freq += 1;
        } else {
            hmap.insert(tok, 1);
        }
    }
}

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
fn c_to_s<'a>(c: &'a [char]) -> String {
    c.iter().collect::<String>()
}
type TFMap = HashMap<String, i32>;
type DirMap = HashMap<String, TFMap>;
const PARSE_LIMIT: i8 = 10;
fn main() {
    let dir_path = "docs.gl/gl4/";
    let dir = fs::read_dir(dir_path).expect("cant read dir");
    let mut dir_hmap: DirMap = HashMap::new();

    for file in dir {
        // limit max files parse
        if dir_hmap.len() == PARSE_LIMIT as usize {
            return;
        }

        // parse 1 file to tfmap
        let file = file.expect("Cant read file");
        let file_path = file.path().to_string_lossy().into_owned();

        let content = read_all_xml(&file_path)
            .expect("cant read file")
            .chars()
            .collect::<Vec<_>>();
        let parser = Lexer::new(&content);
        let mut tfmap: TFMap = HashMap::new();

        for token in parser {
            let token = c_to_s(token);
            Lexer::to_hash_map(token, &mut tfmap);
        }

        // index parsed tfmap
        dir_hmap.insert(file_path.clone(), tfmap);
        println!("{} parsed", file_path);
    }
}
