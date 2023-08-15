use super::*;

#[derive(Serialize, Deserialize,Debug, Clone)]
struct DataUnitaire {
    name: String,
    namepipe: Vec<String>,
}


#[derive(Serialize, Deserialize,Debug)]
pub struct NamePipeArtefacts {
    dictionary: Vec<DataUnitaire>,
}

impl NamePipeArtefacts {
    pub fn init()-> NamePipeArtefacts{
        let file_path = std::env::current_dir().expect("Failed to get current folder").join("data/namepipe.json");
        let json_open = std::fs::File::open(file_path).expect("Unable to open json file");
        let data:NamePipeArtefacts = serde_json::from_reader(json_open).expect("error while reading or parsing the json");
        return data;
    }

    pub fn get_all_name(&self)->Vec<String> {
        let data:Vec<String>  = self.dictionary.clone().into_iter().map(|x| x.name).collect();
        return data
    }

    fn get_malware_index_by_name(&self,name:&String) -> usize{
        let barrox_string= name.to_owned();
        let index = self.dictionary.clone().iter().position(|r| r.name == barrox_string).unwrap();
        return index;
    }

    pub fn get_all_pipename(&self,name:&String)->Vec<String>{
        let index = self.get_malware_index_by_name(name);
        let data  = self.dictionary.get(index).unwrap();
        return data.namepipe.to_owned();
    }

    pub fn get_pipename_by_index(&self,name:&String,number:usize)->String{
        //let list_pipe        
        let mut index_payload = number;
        let list_name_pipe = self.get_all_pipename(&name);
        //Don't trust humain
        if index_payload > list_name_pipe.len(){
            index_payload = 0;
        }
        let barrow = &list_name_pipe[index_payload];
        return barrow.to_owned();

    }
}