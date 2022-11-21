#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let result = std::fs::read_to_string("test2.txt");
//
    //let content = match result {
        //Ok(content) => { content },
        //Err(error) => { return Err(error.into()); }
    //};
    //println!("file content: {}", content);
    //Ok(())
//}


use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
