pub struct Paths{
    paths: Vec<String>
}

impl Paths{
    pub fn new() -> Paths{
        Paths{paths: vec!{"/".to_string()}}
    }

    pub fn is_exists(&self, path: &String) -> bool{
        self.paths.contains(path)
    }

    pub fn get_path_from_request(&self, http_request: &Vec<String>) -> String{
        let first_element = http_request.get(0).unwrap();
        let splited = first_element.split(" ").collect::<Vec<&str>>();
        String::from(splited[1])
    }
}
