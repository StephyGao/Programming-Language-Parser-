mod character_stream;
use character_stream::*;

mod token;
use token::*;

mod scanner;
use scanner::*;

mod parser;
use parser::*;

// mod test;
// use test::*;


fn main() {

    let mut ps = Parser::new("./example1.x");
    ps.check();
    ps.write_html();
}

