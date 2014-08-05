pub fn compile<'a>(tokens: Vec<&'a str>) {
    for (index, token) in tokens.iter().enumerate() {
        println!("{}", token);
    }
}
