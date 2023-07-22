use std::{collections::HashMap, env, fs, process};

struct Response {
    body: String,
}

#[derive(Clone)]
struct Request {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 0 {
        println!("bad arguments");
        process::exit(1);
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
fn parse_request_file(req_file: String) -> Request {
    let req_data = fs::read_to_string(req_file).expect("not able to read the file");
    let parts = req_data.split("\n");

    Request {
        method: "GET".to_string(),
    }
}

fn send_request(req_file: String) {}
