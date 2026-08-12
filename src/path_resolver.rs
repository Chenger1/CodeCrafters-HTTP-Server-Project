pub struct Paths{
    paths: Vec<String>
}

impl Paths{
    pub fn new() -> Paths{
        Paths{
            paths: vec!{
                "/".to_string(),
                "/echo/{str}".to_string()
            }
        }
    }

    pub fn is_exists(&self, path: &String) -> bool{
        for key in self.paths.iter(){
            if key.contains(path.as_str()){
                return true;
            }
        }
        false
    }

    pub fn get_path_from_request(&self, http_request: &Vec<String>) -> String{
        let first_element = http_request.get(0).unwrap();
        let path = first_element.split(" ").collect::<Vec<&str>>();
        let splited = path[1].split("/").collect::<Vec<&str>>();
        String::from(splited[1])
    }
}
