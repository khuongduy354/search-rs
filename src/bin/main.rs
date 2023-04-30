use std::{collections::HashMap, fs};

use search_rs::*;
type TFMap = HashMap<String, i32>; //Term-frequency map
type DirMap = HashMap<String, TFMap>; //File-TFMap map
type FFMap = HashMap<String, f32>; //File-Frequency map
const PARSE_LIMIT: i8 = 100;
const DATA_PATH: &str = "gl4-datasettest.json";

fn main() {
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
}
