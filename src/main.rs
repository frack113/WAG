//
// Windows Artefact Generator POC
//

mod generator;
mod namepipe;

// Cli option 
use clap::{Parser, Subcommand};
mod cli;

// Json
extern crate serde;
use serde::{Deserialize, Serialize};

fn main() -> ! {

    let my_cli = cli::WagCli::parse();

    match my_cli.command {

        cli::Clioptions::NamePipe { name,number,list } => {
            let test= namepipe::NamePipeArtefacts::init();
            let all_name = test.get_all_name();

            if all_name.contains(&name)== false{

                println!("Did not find \"{}\" name for namepipe",name);
                println!("You can use the help option --help");
                std::process::exit(1)

            } else {

                if list == true {
                    println!("Name Pipe number for \"{}\" :",name);
                    println!("----------------");
                    let list_name_pipe = test.get_all_pipename(&name);
                    for i in 0..list_name_pipe.len(){
                        println!(" {} - {}",i,list_name_pipe[i])
                    }
                    println!("----------------");
                    println!("bye");
                    std::process::exit(0)
                } else {
                    //let malware = artefact_data.get_data_by_name(name);
                    let payload= &test.get_pipename_by_index(&name, number);
                    let full_payload = generator::regex_to_string(payload);
                    println!("Create the namepipe : {}",full_payload);
                    generator::create_name_pipe(&full_payload, 2000);
                    std::process::exit(0)
                }

            }// else 

        }, //Namepipe option

        cli::Clioptions::BYOVD { name, details, path } => {
            println!("Bring Your Own Vulnerable Driver");
            // no check you need to be admin :)
            // no check path is valid :)
            let result = generator::create_driver_service(name, details, path);
            if result {
                println!("All is good");
                std::process::exit(0)
            } else{
                println!("Nothing is good");
                std::process::exit(2)
            }
            
        },

    }
    
}
