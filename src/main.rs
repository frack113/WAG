//
// Windows Artefact Generator POC
//

mod aft_filecreate;
mod aft_namepipe;
mod tools_cli;
mod tools_generator;

use clap::Parser;
use std::collections::HashMap;

// std::process::exit need a i32 
const EXIST_ALL_GOOD:i32= 0;
const EXIST_CLI_ERROR:i32 = 10;
const EXIST_TEST_ERROR:i32 = 11;


fn main() -> ! {

    let my_cli = tools_cli::WagCli::parse();

    match my_cli.command {

        tools_cli::Clioptions::BYOVD { name, details, path } => {
            println!("Bring Your Own Vulnerable Driver");
            // no check you need to be admin :)
            // no check path is valid :)
            let result = tools_generator::create_driver_service(name, details, path);
            if result {
                println!("Bye");
                std::process::exit(EXIST_ALL_GOOD)
            } else{
                println!("Sorry get a error");
                std::process::exit(EXIST_TEST_ERROR)
            }
            
        },

        tools_cli::Clioptions::FileCreateAuto { name ,mimic}=>{
            println!("Create a file on disk");
            let mut artefact = aft_filecreate::FileArtefac::new(HashMap::new(),HashMap::new());
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
                std::process::exit(EXIST_ALL_GOOD)
            }

            if artefact.file_payload_exist(&name) == false{
                println!("Did not find \"{}\" name for filecreate",name);
                println!("You can use the help option --help");
                std::process::exit(EXIST_CLI_ERROR)
            } 

            let fullename = artefact.file_payload_getfilename(&name);
            let payload_type = artefact.file_payload_getfiletype(&name);
            let admin = artefact.file_payload_needroot(&name);
            let payload = artefact.file_magicbyte_get(&payload_type);

            if admin && !tools_generator::process_is_admin() {
                println!("Need to have Administrator right to create the file");
                std::process::exit(EXIST_TEST_ERROR)
            }

            let ret = tools_generator::create_file(fullename, payload);
            if ret == true{
                std::process::exit(EXIST_ALL_GOOD)
            }else{
                std::process::exit(EXIST_TEST_ERROR)
            }
        },

        tools_cli::Clioptions::FileCreateManual {filename,magicbyte,list}=>{
            println!("Create a file on disk");
            let mut artefact = aft_filecreate::FileArtefac::new(HashMap::new(),HashMap::new());
            artefact.load("data/files.json");

            if list == true {
                let all_name = artefact.file_magicbyte_list();
                println!("Name for the mimic File creation :");
                println!("----------------");
                for name in all_name {
                    println!(" - {}",name);
                }
                println!("----------------");
                std::process::exit(EXIST_ALL_GOOD)
            }

            if artefact.file_magicbyte_exist(&magicbyte) == false{
                println!("Did not find \"{}\" name for MagicBytes Option",magicbyte);
                println!("You can use the help option --help");
                std::process::exit(EXIST_CLI_ERROR)
            }

            if filename.len() >0 {
                println!("Get the regex : {}",filename);
                let fullname = tools_generator::regex_to_string(&filename);
                let payload = artefact.file_magicbyte_get(&magicbyte);
                let ret = tools_generator::create_file(fullname, payload);
                if ret == true{
                    std::process::exit(EXIST_ALL_GOOD)
                }else{
                    std::process::exit(EXIST_TEST_ERROR)
                }
            }

            std::process::exit(EXIST_CLI_ERROR)
        },

        tools_cli::Clioptions::NamePipeAuto { name,number,pipe , mimic} => {
            let mut artefact= aft_namepipe::NamePipeArtefact::new(HashMap::new());
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
                std::process::exit(EXIST_ALL_GOOD)
            }

           
            if artefact.namepipe_exist(&name) == false {
                println!("Did not find \"{}\" name for namepipe",name);
                println!("You can use the help option --help");
                std::process::exit(EXIST_CLI_ERROR)
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
                std::process::exit(EXIST_ALL_GOOD)
            }

            let payload= artefact.namepipe_get_value_at_index(&name, number);
            let full_payload = tools_generator::regex_to_string(&payload);
            println!("Create the namepipe : {}",full_payload);
            tools_generator::create_name_pipe(&full_payload, 2000);
            std::process::exit(EXIST_ALL_GOOD)

        }, //Namepipe option
        tools_cli::Clioptions::NamePipeManual { name } =>{
            println!("PipeName Artefact");
            let full_payload = tools_generator::regex_to_string(&name);
            println!("Create the namepipe : {}",full_payload);
            tools_generator::create_name_pipe(&full_payload, 2000);
            std::process::exit(EXIST_ALL_GOOD)
        },

    }
    
}
