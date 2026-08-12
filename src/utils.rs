pub fn get_path_parameters(http_request: &Vec<String>) -> Vec<&str> {
    http_request[0].split(" ").collect::<Vec<&str>>()[1]
        .split("/")
        .collect::<Vec<&str>>()
}