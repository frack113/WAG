//
// Windows Artefact Generator POC
//

mod generator;

// Cli option 
use clap::Parser;

// Json
extern crate serde;
use serde::{Deserialize, Serialize};


const JSON_FOLDER: &'static str ="data";

#[derive(Parser)]
#[command(author="Me", version, about, long_about = None)]
struct CliArg {
    // name of the test
    #[arg(short, long,help = "Name of the malware artefact")]
    name: Option<String>,

    #[arg(short, long,help = "List all the malware Name")]
    list: bool,
}


#[derive(Serialize, Deserialize,Clone)]
struct TestData {
    name: String,
    namepipe: Vec<String>,
}

impl TestData{
    fn new() -> Self{
        Self { name: String::new(), namepipe: Vec::new() }
    }
}


struct Artefact{
    test_data: Vec<TestData>,
    list_test: Vec<String>,
}

impl Artefact {
    fn new() -> Self{
        Self{
            test_data: Vec::new(),
            list_test: Vec::new(),
        }
    }

    fn load(&mut self){
        let json_path = std::env::current_dir().expect("Failed to get current folder").join(JSON_FOLDER);
        let json_files = std::fs::read_dir(json_path).expect("Faild to read Json folder");
        for json_file in json_files{
            match json_file{
                Ok(file) => {
                    let file_path = file.path();
                    let json_open = std::fs::File::open(file_path).expect("Unable to open json file");
                    let malware_info:TestData = serde_json::from_reader(json_open).expect("error while reading or parsing the json");

                    let barrow_name_test = &malware_info.name.to_owned();
                    let barrow_malware_info = malware_info;

                    self.test_data.push(barrow_malware_info);
                    self.list_test.push(barrow_name_test.to_string());
                },
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    fn name_exist(&self,name:&String) -> bool{
        if self.list_test.contains(name){
            true
        } else {
            false
        }
    }

    fn get_data_by_name(&self,name:&String) -> &TestData{
        let index = self.list_test.iter().position(|r| r == name).unwrap();
        return &self.test_data[index];
    }

}



fn main() {
    // Get some global variable    
    let mut test_name:String = String::new();
    let mut malware= &TestData::new();

    //We need to load the database at startup
    let mut artefact_data = Artefact::new();
    artefact_data.load();

    // build the cli
    let cli = CliArg::parse();

    // want the list only
    if cli.list{
        println!("Welcome you can use the test name :");
     
        for name in artefact_data.list_test{
            println!(" - {}",name);
        }
     
        std::process::exit(0);
    }

    // check if we get a valid test name
    if cli.name.is_some(){
        test_name = cli.name.unwrap();

        if artefact_data.name_exist(&test_name){
            
            malware = artefact_data.get_data_by_name(&test_name);

        } else {
            println!("Please check the name of the test you want to run");
            println!("Use the list option -l");
            std::process::exit(0)
        }
        
    }

    //
    // Build the name pipe form regex 
    //
    if malware.namepipe.len() >0 {
        println!("Find {} name pipe",malware.namepipe.len());
        for payload in  malware.namepipe.iter() {
            let full_payload = generator::regex_to_string(payload);
            println!("Try name pipe : {}", full_payload);
            generator::create_name_pipe(&full_payload,2000);
        }
    }

}