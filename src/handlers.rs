use crate::utils::{get_path_parameters, get_headers};

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
    let headers = get_headers(http_request);
    for header in headers{
        let data = header.split(":").collect::<Vec<&str>>();
        if data[0] == "User-Agent"{
            response.push_str(&format!("Content-Length: {}\r\n\r\n{}\r\n\r\n", data[1].len(), data[1]));
            break;
        }
    }

    response
}
