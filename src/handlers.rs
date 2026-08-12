pub fn echo(http_request: &Vec<String>) -> String {
    let mut response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n");
    let request_parameters = http_request[0].split(" ").collect::<Vec<&str>>()[1]
        .split("/")
        .collect::<Vec<&str>>();
    if request_parameters.len() != 3{
        return response;
    }
    let body = request_parameters[2];
    response.push_str(&format!("Content-Length: {}\r\n\r\n{}\r\n\r\n", body.len(), body));
    response
}
