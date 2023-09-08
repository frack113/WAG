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
const EXIST_ALL_GOOD: i32 = 0;
const EXIST_CLI_ERROR: i32 = 10;
const EXIST_TEST_ERROR: i32 = 11;

fn pretty_print_hashset(title: String, data: HashSet<String>) {
    println!("{} :", title);
    println!("----------------");
    for name in data {
        println!("👉 {}", name);
    }
    println!("----------------");
}

fn banner() {
    let banner = "

    ██     ██  █████   ██████  
    ██     ██ ██   ██ ██       
    ██  █  ██ ███████ ██   ███ 
    ██ ███ ██ ██   ██ ██    ██ 
     ███ ███  ██   ██  ██████ 
    
    ";
    println!("{}", banner);
}

/* Version 20230908 */
fn run_ads(module: String, get: bool, filename: String) -> i32 {
    println!("Alternate Data Stream");
    let mut artefact = aft_filecreate::FileArtefac::new();
    artefact.load("data/files.json");

    if get == true {
        let all_name = artefact.file_ads_list();
        pretty_print_hashset("Name for the ADS File data".to_string(), all_name);
        return EXIST_ALL_GOOD;
    }

    if artefact.file_ads_exist(&module) == false {
        println!("Did not find \"{}\" name for ads", module);
        println!("You can use the help option --help");
        return EXIST_CLI_ERROR;
    }

    if filename.len() > 0 {
        println!("Get the regex : {}", filename);
        let fullname = tools_generator::regex_to_string(&filename);
        println!("Create the ADS");
        let name_ads = artefact.file_ads_get_name(&module);
        let payload = artefact.file_ads_get_data(&module);
        let ret_ads = tools_generator::create_ads(fullname, name_ads, payload);
        if ret_ads == true {
            return EXIST_ALL_GOOD;
        } else {
            return EXIST_TEST_ERROR;
        }
    }

    EXIST_CLI_ERROR
}

/* Version 20230908 */
fn run_byovd(internal: String, display: String, path: String) -> i32 {
    println!("Bring Your Own Vulnerable Driver");

    if tools_generator::process_is_admin() == false {
        return EXIST_TEST_ERROR;
    }

    // Todo check path is valid or not :)

    let result = tools_generator::create_driver_service(internal, display, path);
    if result {
        println!("All good ");
        return EXIST_ALL_GOOD;
    } else {
        println!("Sorry get a error");
        return EXIST_TEST_ERROR;
    }
}

/* Version 20230908 */
fn run_createfile(
    module: String,
    get: bool,
    filename: String,
    magicbyte: String,
    details: bool,
) -> i32 {
    println!("Create a file on disk");
    let mut artefact = aft_filecreate::FileArtefac::new();
    artefact.load("data/files.json");

    if get == true {
        let all_name = artefact.file_payload_list();
        pretty_print_hashset("Name for the mimic File creation".to_string(), all_name);
        return EXIST_ALL_GOOD;
    }

    let fullname: String;
    let payload: Vec<u8>;

    if module == "manual" {
        if details == true {
            let all_name = artefact.file_magicbyte_list();
            pretty_print_hashset("Name for the MagicByte File creation".to_string(), all_name);
            return EXIST_ALL_GOOD;
        }

        if artefact.file_magicbyte_exist(&magicbyte) == false {
            println!("Did not find \"{}\" name for MagicBytes Option", magicbyte);
            println!("You can use the help option --help");
            return EXIST_CLI_ERROR;
        }

        if filename.len() > 0 {
            println!("Get the regex : {}", filename);
            fullname = tools_generator::regex_to_string(&filename);
            payload = artefact.file_magicbyte_get(&magicbyte);
        } else {
            return EXIST_CLI_ERROR;
        }
    } else {
        if artefact.file_payload_exist(&module) == false {
            println!("Did not find \"{}\" name for filecreate", module);
            println!("You can use the help option --help");
            return EXIST_CLI_ERROR;
        }

        let payload_type = artefact.file_payload_getfiletype(&module);
        let admin = artefact.file_payload_needroot(&module);

        fullname = artefact.file_payload_getfilename(&module);
        payload = artefact.file_magicbyte_get(&payload_type);

        if admin && !tools_generator::process_is_admin() {
            println!("Need to have Administrator right to  the file");
            return EXIST_TEST_ERROR;
        }
    }

    let ret = tools_generator::create_file(fullname, payload);

    if ret == true {
        return EXIST_ALL_GOOD;
    } else {
        return EXIST_TEST_ERROR;
    }
}

/* Version 20230908 */
fn run_pipecreate(module: String, number: usize, get: bool, details: bool,name:String) -> i32 {
    println!("Create NamePipe");
    let mut artefact = aft_namepipe::NamePipeArtefact::new();
    artefact.load("data/namepipe.json");

    let full_payload:String;

    if get == true {
        let all_name = artefact.namepipe_list();
        pretty_print_hashset("Name for the mimic Name Pipe".to_string(), all_name);
        return EXIST_ALL_GOOD;
    }
    if module == "manual" {
        if name.len() >0 {
            full_payload = tools_generator::regex_to_string(&name);
        } else {
            return EXIST_CLI_ERROR;
        }
    } else {
        if artefact.namepipe_exist(&module) == false {
            println!("Did not find \"{}\" name for namepipe", module);
            println!("You can use the help option --help");
            return EXIST_CLI_ERROR;
        }

        if details == true {
            println!("Name Pipe number for \"{}\" :", module);
            println!("----------------");
            let list_name_pipe = artefact.namepipe_value_list(&module);
            for i in 0..list_name_pipe.len() {
                println!(" {} - {}", i, list_name_pipe[i])
            }
            println!("----------------");
            println!("bye");
            return EXIST_ALL_GOOD;
        }

        let payload = artefact.namepipe_get_value_at_index(&module, number);
        full_payload = tools_generator::regex_to_string(&payload);

    }

    println!("Create the namepipe : {}", full_payload);
    tools_generator::create_name_pipe(&full_payload, 2000);
    return EXIST_ALL_GOOD;

}

fn main() -> ! {
    banner();
    let my_cli = tools_cli::WagCli::parse();

    match my_cli.command {
        tools_cli::Clioptions::ADS {
            module,
            get,
            filename,
        } => {
            let ret = run_ads(module, get, filename);
            std::process::exit(ret);
        }

        tools_cli::Clioptions::BYOVD {
            internal,
            display,
            path,
        } => {
            let ret = run_byovd(internal, display, path);
            std::process::exit(ret);
        }

        tools_cli::Clioptions::FileCreate {
            module,
            get,
            filename,
            magicbyte,
            details,
        } => {
            let ret = run_createfile(module, get, filename, magicbyte, details);
            std::process::exit(ret);
        }

        tools_cli::Clioptions::NamePipe{
            module,
            number,
            get,
            details,
            name,
        } => {
            let ret= run_pipecreate(module, number, get, details,name);
            std::process::exit(ret);
        } 

    }
}
