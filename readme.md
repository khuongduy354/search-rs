# A search engine in Rust  
- Input a bunch of words
- Search using tf-idf algo 

# Todo 
<!-- - 1 hashmap for 1 file, makes another hashmap that has file-hashmap for all files in docs.gl  -->
<!-- - parse that big hashmap to json   -->

- More lexer features (watch videos), uppercase, dataset
- search words => create hashmap, has path-freq of that words


- Small tricks video?
- WASM  
- 1:21:00 above, heavy algos and computations 


# Learned  
WASM RUST is complicated 
- read file: cannot use rust file::open 
- to use npm with rust is more setup

Hashmap vs BTree 

Array vs Vector of char: 
- char is easier to manipulate than string 
- vector is growable, compared to array 
- array more convenience for slicing.
```rust
    fn chop_left(&self, n: usize) -> Vec<char> {
        let result = self.source[0..n].to_vec(); // slicing return [char]
        self.source = self.source[n..].to_vec();
        result.to_vec()
    }
```

Vector (in global main fn) can be converted into &'a [char] somehow.
```rust 
    fn new(source: &'a [char]) -> Self { 

    //content:Vec<char>;
    let parser = Lexer::new(&content);
```

Implemented iterator for a struct  
```rust   
for thing in baskets{ 
// each iteration of loop call baskets.next() (Iterator trait),  
// if next return None, end loops
}
```

Pointer in hashmap 
```rust 
let value_ref = hmap.get(&key); 
*value_ref+=1; //change value in hmap
```

How rust sort works, cmp and stuffs


# Design decision 
- no cursor needed, since chopping is enough


# Sources
idea from: https://www.youtube.com/watch?v=hm5xOJiVEeg&list=WL&index=16&t=3s
algo: https://en.wikipedia.org/wiki/Tf%E2%80%93idf
documents: https://github.com/BSVino/docs.gl
