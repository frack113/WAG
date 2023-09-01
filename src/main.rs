/*
 ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ _________ ____ ____ ____ ____ ____ ____ ____ ____ ____ 
||W |||i |||n |||d |||o |||w |||s |||       |||A |||r |||t |||e |||f |||a |||c |||t |||       |||G |||e |||n |||e |||r |||a |||t |||o |||r ||
||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||_______|||__|||__|||__|||__|||__|||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/_______\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|/__\|

Working date:  2023-08
*/


mod aft_filecreate;
mod aft_namepipe;
mod tools_cli;
mod tools_generator;

use clap::Parser;
use std::collections::HashSet;

// std::process::exit need a i32 
const EXIST_ALL_GOOD:i32= 0;
const EXIST_CLI_ERROR:i32 = 10;
const EXIST_TEST_ERROR:i32 = 11;

fn pretty_print_hashset(title:String,data:HashSet<String>){
    println!("{} :",title);
    println!("----------------");
    for name in data {
        println!("👉 {}",name);
    }
    println!("----------------");
}

fn main() -> ! {

    let my_cli = tools_cli::WagCli::parse();

    match my_cli.command {

        tools_cli::Clioptions::ADS { filename, adsname, list } =>{
            println!("Alternate Data Stream");
            let mut artefact = aft_filecreate::FileArtefac::new();
            artefact.load("data/files.json");

            if list == true {
                let all_name = artefact.file_ads_list();
                pretty_print_hashset("Name for the ADS File data".to_string(),all_name);
                std::process::exit(EXIST_ALL_GOOD)
            }

            if artefact.file_ads_exist(&adsname) == false{
                println!("Did not find \"{}\" name for ads",adsname);
                println!("You can use the help option --help");
                std::process::exit(EXIST_CLI_ERROR)
            }             

            if filename.len() >0 {
                println!("Get the regex : {}",filename);
                let fullname = tools_generator::regex_to_string(&filename);
                println!("Create the ADS");
                let name_ads = artefact.file_ads_get_name(&adsname);
                let payload = artefact.file_ads_get_data(&adsname);
                let ret_ads = tools_generator::create_ads(fullname,name_ads,payload);
                if ret_ads == true {
                    std::process::exit(EXIST_ALL_GOOD)
                }else{
                    std::process::exit(EXIST_TEST_ERROR)
                }

            }
            std::process::exit(EXIST_CLI_ERROR)
        }

        tools_cli::Clioptions::BYOVD { name, details, path } => {
            println!("Bring Your Own Vulnerable Driver");
            // Todo check be admin :)
            // Todo check path is valid :)
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
            let mut artefact = aft_filecreate::FileArtefac::new();
            artefact.load("data/files.json");

            if mimic == true{
                let all_name = artefact.file_payload_list();
                pretty_print_hashset("Name for the mimic File creation".to_string(),all_name);
                std::process::exit(EXIST_ALL_GOOD)
            }

            if artefact.file_payload_exist(&name) == false{
                println!("Did not find \"{}\" name for filecreate",name);
                println!("You can use the help option --help");
                std::process::exit(EXIST_CLI_ERROR)
            } 

            let payload_type = artefact.file_payload_getfiletype(&name);
            let fullename = artefact.file_payload_getfilename(&name);
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
            let mut artefact = aft_filecreate::FileArtefac::new();
            artefact.load("data/files.json");

            if list == true {
                let all_name = artefact.file_magicbyte_list();
                pretty_print_hashset("Name for the mimic File creation".to_string(),all_name);
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
            let mut artefact= aft_namepipe::NamePipeArtefact::new();
            artefact.load("data/namepipe.json");

            if mimic == true {
                let all_name = artefact.namepipe_list();
                pretty_print_hashset("Name for the mimic Name Pipe".to_string(),all_name);
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
