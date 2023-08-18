//
// File creation Artefact
//
use crate::generator;

use std::collections::{HashMap,HashSet};
use serde::Deserialize;


use std::env;

#[derive(Deserialize)]
struct JsonHeaderInfo {
    name: String,
    magicbyte: String,
}

#[derive(Deserialize)]
struct JsonPayloadInfo {
    name: String,
    file_type:String,
    fullpath: String,
    cmd_var: String,
    cmd_path: String
}


#[derive(Deserialize)]
struct JsonGlobalInfo {
    magicbytes: Vec<JsonHeaderInfo>,
    payloads: Vec<JsonPayloadInfo>,
}

pub struct PayloadPathInfo{
    file_type:String,
    fullpath:String,
    cmd_var:String,
    cmd_path:String,
}

pub struct FileArtefac{
    magicbyte: HashMap<String, String>,
    payload: HashMap<String,PayloadPathInfo>,
}

impl FileArtefac{
    pub fn new(magicbyte: HashMap<String, String>,payload:HashMap<String,PayloadPathInfo> ) -> Self { 
        Self { 
            magicbyte: magicbyte,
            payload:payload
        } 
    }

    pub fn load(&mut self,path:&str){
        let file_path = std::env::current_dir().expect("Failed to get current folder").join(path);
        let json_open = std::fs::File::open(file_path).expect("Unable to open json file");
        let json_data:JsonGlobalInfo = serde_json::from_reader(json_open).expect("error while reading or parsing the json");
    
        for data in json_data.magicbytes{
            self.magicbyte.insert(data.name, data.magicbyte);
        }

        for data in json_data.payloads{
            let tmp_data:PayloadPathInfo = {PayloadPathInfo{file_type:data.file_type,fullpath:data.fullpath, cmd_var:data.cmd_var, cmd_path:data.cmd_path}};
            self.payload.insert(data.name, tmp_data);
        }
    }
/*
    pub fn file_magicbyte_list(&self)->HashSet<String>{
        self.magicbyte.keys().cloned().collect()
    }
*/    
    pub fn file_magicbyte_exist(&self,name:&str)->bool{
        self.magicbyte.contains_key(name)
    }

    pub fn file_magicbyte_get(self,name:&str)-> Vec<u8>{
        if self.file_magicbyte_exist(name){
            let payload = self.magicbyte.get(name).unwrap(); //Can not faild as the key exist
            let header = generator::hex_to_bytes(payload); // User input 😅
            match header{
                Some(data) => data,
                None => vec![0],
            }
        } else {
            vec![0]
        }
    }

    pub fn file_payload_list(&self)->HashSet<String>{
        self.payload.keys().cloned().collect()
    }

    pub fn file_payload_exist(&self,name:&str)->bool{
        self.payload.contains_key(name)
    }

    pub fn file_payload_getfilename(&self,name:&str)->String{
        if self.file_payload_exist(name){
            let data = self.payload.get(name).unwrap(); // name is a valid key
            
            if data.fullpath.len() >0 {
                generator::regex_to_string(&data.fullpath)
            } else {
                let filename = generator::regex_to_string(&data.cmd_path);
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
            "Exe".to_string()
        }
    }
}

