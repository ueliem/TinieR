extern crate rustvm;
use std::str::from_utf8_owned;
use std::io::File;

mod tokenizer;
mod compiler;
mod treetraverser;

fn main() {
    let path = Path::new("test4.tny");
    let mut file = File::open(&path);

    let data = match file.read_to_end() {
    	Ok(n) => n,
	    Err(m) => fail!()
    };
    // println!("{}", compiler::compile(tokenizer::tokenize(std::str::from_utf8(data.as_slice()).unwrap())));
    treetraverser::interpret_tree(box compiler::compile(tokenizer::tokenize(std::str::from_utf8(data.as_slice()).unwrap())));
}
