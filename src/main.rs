use anyhow::Context;
#[allow(unused_imports)]
use flate2::read::{ZlibDecoder};
use std::ffi::CStr;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fmt::format;
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
            
            let mut f = std::fs::File::open(format!(".git/objects/{}/{}",&object_hash[..2], &object_hash[2..])).context("open in .git/objects").unwrap();
            
            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            
            let mut buf = Vec::new();

            z.read_until(0, &mut buf).context("read header from .git/objects").unwrap();
            
            let header = CStr::from_bytes_with_nul(&buf).expect("there is exactly one null at the end.");
            let header= header.to_str();
        }
    }




    
        
}
