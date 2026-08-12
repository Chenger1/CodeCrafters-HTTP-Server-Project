use crate::utils::get_path_parameters;

pub fn echo(http_request: &Vec<String>) -> String {
    let mut response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n");
    let request_parameters = get_path_parameters(http_request);
    if request_parameters.len() != 3{
        return response;
    }
    let body = request_parameters[2];
    response.push_str(&format!("Content-Length: {}\r\n\r\n{}\r\n\r\n", body.len(), body));
    response
}

pub fn user_agent(http_request: &Vec<String>) -> String {
    let mut response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n");
    for line in http_request{
        if line.contains("User-Agent") {
            let data = line.split(":").collect::<Vec<&str>>();
            let value = data[1].trim();
            response.push_str(&format!("Content-Length: {}\r\n\r\n{}\r\n\r\n", value.len(), value));
            break;
        }
    }

    response
}
