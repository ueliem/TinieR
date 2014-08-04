use std::str::from_utf8_owned;
use std::io::File;

mod tokenizer;

fn main() {
    let path = Path::new("test2.tny");
    let mut file = File::open(&path);

    let data = match file.read_to_end() {
    	Ok(n) => n,
	    Err(m) => fail!()
    };
    println!("{}", tokenizer::tokenize(std::str::from_utf8(data.as_slice()).unwrap()));
}
