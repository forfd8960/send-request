use std::{collections::HashMap, env, fs, process};

struct Response {
    code: i32,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Clone)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

const GET: &str = "GET";
const PUT: &str = "PUT";
const POST: &str = "POST";
const DELETE: &str = "DELETE";

impl Request {
    fn send(&self) -> Result<Response, &'static str> {
        match self.method.as_str() {
            GET => {}
            PUT => {}
            DELETE => {}
            POST => {}
            _ => {
                return Err("invalid request method");
            }
        }
        Ok(Response::new(200, HashMap::new(), "".to_string()))
    }
}

impl Response {
    fn new(code: i32, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            code: code,
            headers: headers,
            body: body,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        println!("bad arguments");
        process::exit(1);
    }

    let file = args.get(0).unwrap();

    let request_result = parse_request_file(&file);
    match request_result {
        Ok(req) => {
            let resp = req.send();
        }
        Err(e) => {
            println!("parse request file error: {}", e);
            process::exit(1);
        }
    }

    println!("Hello, world!");
}

/*

the request format is:
method url

header1: value1
header2: value2

{body}
*/
fn parse_request_file(req_file: &str) -> Result<Request, &'static str> {
    let req_data = fs::read_to_string(req_file).expect("not able to read the file");
    let parts = req_data.split("\n");

    let content: Vec<&str> = parts.collect();
    if content.len() < 5 {
        return Err("invalid request data");
    }

    let method_url = parse_method_url(&content);
    let mut method = "".to_string().trim();
    let mut url = "".to_string().trim();
    match method_url {
        Err(e) => return Err(e),
        Ok(v) => {
            method = &v.0;
            url = &v.1;
        }
    }

    let headers = parse_headers(&content[2..]);
    if headers.is_err() {
        return Err(headers.err().unwrap());
    }

    let hrds = headers.unwrap();
    let mut body = "".to_string();
    if 2 + hrds.len() >= content.len() {
        body = "".to_string();
    } else {
        body = parse_body(&content[2 + hrds.len()..]);
    }

    Ok(Request {
        method: method.to_string(),
        url: url.to_string(),
        headers: hrds,
        body: body.to_string(),
    })
}

fn parse_method_url(content: &Vec<&str>) -> Result<(String, String), &'static str> {
    let method_url = content.get(0);

    match method_url {
        None => {
            return Err("invalid request data");
        }
        Some(v) => {
            let tmp: Vec<&str> = v.split(" ").collect();
            if tmp.len() != 2 {
                return Err("invalid method and url");
            }

            let method = tmp.get(0).unwrap();
            let url = tmp.get(1).unwrap();
            Ok((method.trim().to_string(), url.trim().to_string()))
        }
    }
}

fn parse_headers(content: &[&str]) -> Result<HashMap<String, String>, &'static str> {
    let mut res: HashMap<String, String> = HashMap::new();
    for kv in content {
        if kv.len() == 0 {
            break;
        }

        let kv_tmp: Vec<&str> = kv.split(":").collect();
        if kv_tmp.len() != 2 {
            return Err("invlid header");
        }

        res.insert(
            kv_tmp.get(0).unwrap().trim().to_string(),
            kv_tmp.get(1).unwrap().trim().to_string(),
        );
    }

    Ok(res)
}

fn parse_body(content: &[&str]) -> String {
    let body_content = content.join("\n");
    let body = body_content.trim();
    body.to_string()
}
