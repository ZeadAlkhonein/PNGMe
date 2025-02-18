mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use std::env;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main(){
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    
    
    // let config: Config = Config::build(&args).unwrap_or_else(|err|{
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // let _ = run(config)?;

    // another way to handle error 
    //     if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    // Ok(())
}