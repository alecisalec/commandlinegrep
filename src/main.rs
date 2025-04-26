use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    

    let query = &args[1];
    let fp = &args[2];

    println!("searching query {}", query);
    println!("In file: {}", fp);

    let contents = fs::read_to_string(fp)
        .expect(" error, should have been a file");

    println!("contents:\n {}", contents);


    // fn parse_config();

}
