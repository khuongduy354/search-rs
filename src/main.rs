// use std::path::PathBuf;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
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
type TFMap = HashMap<String, i32>; //Term-frequency map
type DirMap = HashMap<String, TFMap>; //File-TFMap map
type FFMap = HashMap<String, f32>; //File-Frequency map
const PARSE_LIMIT: i8 = 10;
const DATA_PATH: &str = "gl4-datasettest.json";

fn test_program() {
    let mut map = HashMap::new();
    // map.insert("name", "John Doe");
    let mut map2 = HashMap::new();
    map2.insert("name", 3);
    map2.insert("name2", 4);

    let mut map3 = HashMap::new();
    map2.insert("name", 3);
    map2.insert("name2", 4);
    map3.insert("name2", 4);
    // map.insert("name", "Duy");
    map.insert("name3", map3);
    map.insert("name2", map2);
    // map.insert("age", 30);

    let json = serde_json::to_string(&map).unwrap();
    println!("{}", json);
}
fn tf_compute(term: &str, tfmap: TFMap) -> f32 {
    let a = tfmap.get(term).unwrap_or_else(|| &0).to_owned() as f32;
    let b = tfmap.values().sum::<i32>() as f32;
    a / b
}
fn parse_to_ffmap(term: String) -> Result<FFMap, std::io::Error> {
    let file = File::open(DATA_PATH)?;
    let reader = BufReader::new(file);
    let data: String = serde_json::from_reader(reader)?;
    let data = serde_json::from_str(&data)?;

    let mut result: FFMap = HashMap::new();
    let dirmap = load_json_to_hashmap()?;
    for (path, tfmap) in dirmap {
        result.insert(path, tf_compute(&term, tfmap));
    }
    Ok(result)
}
fn test2program() {
    let dir_path = "docs.gl/gl4/";
    let dir = fs::read_dir(dir_path).expect("cant read dir");
    let mut dir_hmap: DirMap = HashMap::new();

    for file in dir {
        // limit max files parse
        if dir_hmap.len() == PARSE_LIMIT as usize {
            break;
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

    let result = parse_dir_to_json(&dir_hmap).expect("cant parse");
    // println!("{}", result);
    save_json_to_disk(result, "gl4-datasettest.json");
    // let hmp = load_json_to_hashmap().expect("no json");
    // let newjs = parse_dir_to_json(&hmp).expect("cant parse");
    // save_json_to_disk(newjs, "gl4-datasettest2.json");
}
fn main_program() {
    let dir_path = "docs.gl/gl4/";
    let dir = fs::read_dir(dir_path).expect("cant read dir");
    let mut dir_hmap: DirMap = HashMap::new();

    for file in dir {
        // limit max files parse
        if dir_hmap.len() == PARSE_LIMIT as usize {
            break;
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
    let result = parse_dir_to_json(&dir_hmap).expect("cant parse");
    // println!("{}", result);
    save_json_to_disk(result, "gl4-dataset.json");
}

// JSON HELPERS ===>

fn save_json_to_disk(data: String, filename: &str) {
    let mut file = File::create(filename).expect("Cant create");
    file.write_all(data.as_bytes()).expect("cant write");
    println!("Saved to disk!");
}

fn parse_dir_to_json(
    hmap: &HashMap<String, HashMap<String, i32>>,
) -> Result<String, std::io::Error> {
    let json = serde_json::to_string_pretty(hmap)?;
    Ok(json)
}

fn load_json_to_hashmap() -> Result<DirMap, std::io::Error> {
    let data = serde_json::from_reader(File::open(DATA_PATH)?)?;
    let mut hmap: DirMap = HashMap::new();

    if let Value::Object(inner) = data {
        for (key, value) in inner {
            match value {
                Value::Object(_inner) => {
                    let mut tf_map: TFMap = HashMap::new();
                    for (_key, _value) in _inner {
                        if let Value::Number(freq) = _value {
                            tf_map.insert(_key, freq.as_i64().expect("Cant cast number") as i32);
                        }
                    }
                    hmap.insert(key, tf_map);
                }
                _ => (),
            }
        }
    }
    Ok(hmap)
}

fn main() {
    // test2program();
    // main_program();
}
