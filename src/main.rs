//
// Windows Artefact Generator POC
//


// Cli option 
use clap::Parser;
mod cli;


mod generator;
mod namepipe;
mod filedrop;


use std::collections::HashMap;

fn main() -> ! {

    let my_cli = cli::WagCli::parse();

    match my_cli.command {

        cli::Clioptions::NamePipe { name,number,pipe , mimic} => {
            let mut artefact= namepipe::NamePipeArtefact::new(HashMap::new());
            artefact.load("data/namepipe.json");

            if mimic == true {
                let all_name = artefact.namepipe_list();
                println!("Name for the mimic Name Pipe :");
                println!("----------------");
                for mimic_name in all_name{
                    println!(" - {}", mimic_name);
                }
                println!("----------------");
                println!("bye");
                std::process::exit(0)
            }

           
            if artefact.namepipe_exist(&name) == false{
                println!("Did not find \"{}\" name for namepipe",name);
                println!("You can use the help option --help");
                std::process::exit(1)
            } 

            if pipe == true {
                println!("Name Pipe number for \"{}\" :",name);
                println!("----------------");
                let list_name_pipe = artefact.namepipe_value_list(&name);
                for i in 0..list_name_pipe.len(){
                    println!(" {} - {}",i,list_name_pipe[i])
                }
                println!("----------------");
                println!("bye");
                std::process::exit(0)
            }

            let payload= artefact.namepipe_get_value_at_index(&name, number);
            let full_payload = generator::regex_to_string(&payload);
            println!("Create the namepipe : {}",full_payload);
            generator::create_name_pipe(&full_payload, 2000);
            std::process::exit(0)

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

        cli::Clioptions::Filecreate { name ,mimic}=>{
            println!("Create a file on disk");
            let mut artefact = filedrop::FileArtefac::new(HashMap::new(),HashMap::new());
            artefact.load("data/files.json");

            if mimic == true{
                let all_name = artefact.file_payload_list();
                println!("Name for the mimic File creation :");
                println!("----------------");
                for name in all_name {
                    println!(" - {}",name);
                }
                println!("----------------");
                println!("bye");
                std::process::exit(0)
            }

            if artefact.file_payload_exist(&name) == false{
                println!("Did not find \"{}\" name for filecreate",name);
                println!("You can use the help option --help");
                std::process::exit(1)
            } 

            let fullename = artefact.file_payload_getfilename(&name);
            let payload_type = artefact.file_payload_getfiletype(&name);
            let payload = artefact.file_magicbyte_get(&payload_type);
            generator::create_file(fullename, payload);
            std::process::exit(0)
        }
    }
    
}
