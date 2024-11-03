#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
// use clap::Command;
use clap::{Parser,Subcommand};

#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
struct Args{
#[command(subcommand)]
command:Command,

}

#[derive(Debug,Subcommand)]
enum Command{
    Init,
}


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    let args= Args::parse();

    match args.command{
        Command::Init=>{
            fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
        }
        _=>{
            
        }
    }


   
        
}
