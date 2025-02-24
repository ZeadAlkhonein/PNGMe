use std::{env, process};

use pngme::{args::Config, operation};
// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;


fn main(){
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error building config: {}", err);
        process::exit(1);
    });
    let _operation_result = operation(&config).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        process::exit(1);
    });



    // let _ = run(config)?;

    // another way to handle error 
    //     if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    // Ok(())
}