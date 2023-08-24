/*
        File creation Artefact

the magic bytes and NTFS ADS are store a string HEX value.
like 504B0304 or 4D5A.

Path are regex in json so need `\\.` to get a `.`.

if fullpath is empty , it build the path from
env variable and the cmd_path.
"SystemRoot" "Temp\\debug\\.bin" will give "c:\Windows\Temp\debug.bin"

You can use `SET | more` or `Get-ChildItem Env:` to get the list

*/

use crate::tools_generator;

use std::collections::{HashMap,HashSet};
use serde::Deserialize;

use std::env;

/*
    Json Data structure
*/
#[derive(Deserialize)]
struct JsonHeaderInfo {
    name: String,
    magicbyte: String,
}

#[derive(Deserialize)]
struct JsonPayloadInfo {
    name: String,
    needroot: bool,
    file_type:String,
    fullpath: String,
    cmd_var: String,
    cmd_path: String
}

#[derive(Deserialize)]
struct JsonAdsInfo {
    name: String,
    adsname: String,
    hexvalue: String,
}

#[derive(Deserialize)]
struct JsonGlobalInfo {
    magicbytes: Vec<JsonHeaderInfo>,
    payloads: Vec<JsonPayloadInfo>,
    ads: Vec<JsonAdsInfo>,
}

/*
    Rust internal Structure
    Get error with "copy"

    split ads struct into 2 Hashmap
*/
pub struct PayloadPathInfo{
    needroot: bool,
    file_type:String,
    fullpath:String,
    cmd_var:String,
    cmd_path:String,
}


pub struct FileArtefac{
    magicbyte: HashMap<String, String>,
    payload: HashMap<String,PayloadPathInfo>,
    ads_adsname: HashMap<String, String>,
    ads_hexvalue: HashMap<String, String>,
}

impl FileArtefac{
    pub fn new( ) -> Self { 
        Self { 
            magicbyte: HashMap::new(),
            payload:HashMap::new(),
            ads_adsname:HashMap::new(),
            ads_hexvalue:HashMap::new()
        } 
    }

    pub fn load( &mut self,path:&str){
        let file_path = std::env::current_dir().expect("Failed to get current folder").join(path);
        let json_open = std::fs::File::open(file_path).expect("Unable to open json file");
        let json_data:JsonGlobalInfo = serde_json::from_reader(json_open).expect("error while reading or parsing the json");
    
        for data in json_data.magicbytes{
            self.magicbyte.insert(data.name, data.magicbyte);
        }

        for data in json_data.payloads{
            let tmp_data:PayloadPathInfo = {PayloadPathInfo{needroot:data.needroot,file_type:data.file_type,fullpath:data.fullpath, cmd_var:data.cmd_var, cmd_path:data.cmd_path}};
            self.payload.insert(data.name, tmp_data);
        }

        for data in json_data.ads{
            self.ads_adsname.insert(data.name.clone(), data.adsname);
            self.ads_hexvalue.insert(data.name,data.hexvalue);
        }

    }

    pub fn file_magicbyte_list(&self)->HashSet<String>{
        self.magicbyte.keys().cloned().collect()
    }
   
    pub fn file_magicbyte_exist(&self,name:&str)->bool{
        self.magicbyte.contains_key(name)
    }

    pub fn file_magicbyte_get(&self,name:&str)-> Vec<u8>{
        let mut payload = "467261636b313133";
        if self.file_magicbyte_exist(name){
            payload = self.magicbyte.get(name).clone().unwrap(); //Can not faild as the key exist
        }
        let header = tools_generator::hex_to_bytes(payload); // User input 😅
        match header{
            Some(data) => data,
            None => vec![70,114,97,99,107,49,49,51],
        }
    }

    pub fn file_payload_list(&self)->HashSet<String>{
        self.payload.keys().cloned().collect()
    }

    pub fn file_payload_exist(&self,name:&str)->bool{
        self.payload.contains_key(name)
    }

    pub fn file_payload_needroot(&self,name:&str)->bool{
        let data = self.payload.get(name).unwrap(); // exit fail if name is invalid 
        data.needroot
    }

    pub fn file_payload_getfilename(&self,name:&str)->String{
        if self.file_payload_exist(name){
            let data = self.payload.get(name).unwrap(); // name is a valid key
            
            if data.fullpath.len() >0 {
                tools_generator::regex_to_string(&data.fullpath)
            } else {
                let filename = tools_generator::regex_to_string(&data.cmd_path);
                let var_path= env::var_os(&data.cmd_var).unwrap();
                let full_path = std::path::Path::new(&var_path).join(filename);
                String::from(full_path.to_string_lossy())
            }
            
        } else {
            "c:/wag.file".to_string()
        }
    }

    pub fn file_payload_getfiletype(&self,name:&str)->String{
        if self.file_payload_exist(name){
            let data = self.payload.get(name).unwrap(); // name is a valid key
            data.file_type.clone()
                        
        } else {
            "Wag".to_string()
        }
    }

    pub fn file_ads_list(&self)->HashSet<String>{
        self.ads_adsname.keys().cloned().collect()
    }

    pub fn file_ads_exist(&self,name:&str)->bool{
        self.ads_adsname.contains_key(name)
    }

    pub fn file_ads_get_data(&self,name:&str)->Vec<u8>{
        println!("Ask for {}",name);
        let payload = self.ads_hexvalue.get(name).clone().unwrap(); //Can not faild as the key exist
        println!("{}",payload);
        let header = tools_generator::hex_to_bytes(payload); // User input 😅
        match header{
            Some(data) => data,
            None => vec![70,114,97,99,107,49,49,51],
        }
    }

    pub fn file_ads_get_name(&self,name:&str)->String{
        let data = self.ads_adsname.get(name).clone().unwrap(); //Can not faild as the key exist
        data.to_string()
    }

}
