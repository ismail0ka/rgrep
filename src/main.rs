use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};
use clap::Parser;


#[derive(Parser)]
struct CliArgs{
    pattern: String,
    path: PathBuf
}
fn main() {
    let args = CliArgs::parse();
    
    let content = File::open(&args.path).expect("Could not open file");
    let lines = BufReader::new(content).lines();
    for line in lines{
        let l = match line{
            Ok(l) => l ,
            Err(err) => panic!("{}",err),
        };
        if l.contains(&args.pattern){
            println!("{}",l);
        }
    }
}
