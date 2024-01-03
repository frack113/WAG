/*
 ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ ____
||W |||i |||n |||d |||o |||w |||s |||       |||A |||r |||t |||e |||f |||a |||c |||t |||       |||G |||e |||n |||e |||r |||a |||t |||o |||r ||
||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|

Working date:  2023-08
*/

use wag::cli::Arguments;

use clap::Parser;

fn banner() {
    let banner: &str = "

    ██     ██  █████   ██████  
    ██     ██ ██   ██ ██       
    ██  █  ██ ███████ ██   ███ 
    ██ ███ ██ ██   ██ ██    ██ 
     ███ ███  ██   ██  ██████ 
    
    ";
    println!("{}", banner);
}

fn main() -> () {
    banner();

    match Arguments::try_parse() {
        Ok(arguments) => std::process::exit(arguments.run()),
        Err(error) => {
            error.exit();
        }
    }
}
