# A search engine in Rust  
- Input a bunch of words
- Search using tf-idf algo 

# Learned 
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


# Design decision 
- no cursor needed, since chopping is enough


# Sources
idea from: https://www.youtube.com/watch?v=hm5xOJiVEeg&list=WL&index=16&t=3s
algo: https://en.wikipedia.org/wiki/Tf%E2%80%93idf
documents: https://github.com/BSVino/docs.gl
