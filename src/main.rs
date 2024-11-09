use anyhow::Context;

use clap::{Parser, Subcommand};

use std::env;

use flate2::read::ZlibDecoder;

use std::ffi::CStr;

use std::fs;

use std::io;

use std::io::prelude::*;

use std::io::BufReader;

/// Simple program to greet a person

#[derive(Parser, Debug)]

#[command(version,about,long_about=None)]
struct Args{
#[command(subcommand)]
command:Command,

}

#[derive(Debug,Subcommand)]
enum Command{
    Init,
    CatFile{
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    }
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
        Command::CatFile { pretty_print, object_hash }=>{
            
            // anyhow::ensure!(pretty_print,"mode must be given without -p, we dont supprot mode ");
            
        let content = fs::read(format!(".git/objects/{}/{}",&object_hash[..2], &object_hash[2..])).unwrap();

        let mut z = ZlibDecoder::new(&content[..]);

        let mut s = String::new();

        z.read_to_string(&mut s).unwrap();

        print!("{}", &s[8..]);
        }
    }




    
        
}
