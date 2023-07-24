use reqwest;
use reqwest::blocking::Client;
use std::{collections::HashMap, env, fs, process};

struct Response {
    code: i32,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Clone)]
struct Request<'a> {
    method: &'a str,
    url: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
    req_data: String,
    content: &'a [&'a str],
}

const GET: &str = "GET";
const PUT: &str = "PUT";
const POST: &str = "POST";
const DELETE: &str = "DELETE";

impl<'a> Request<'a> {
    fn new() -> Self {
        Self {
            method: "",
            url: "",
            headers: HashMap::new(),
            body: "",
            req_data: "".to_string(),
            content: &[],
        }
    }
    /*

    the request format is:
    method url

    header1: value1
    header2: value2

    {body}
    */
    fn from_file(&mut self, file: String) -> Result<Self, &'a str> {
        self.read_file_content(file);
        let method_url = self.parse_method_and_url();
        match method_url {
            Err(e) => return Err(e),
            Ok(v) => {
                self.method = &v.0.clone();
                self.url = &v.1.clone();
            }
        }

        let headers = self.parse_headers();
        match headers {
            Err(e) => return Err(e),
            Ok(v) => {
                self.headers = v;
            }
        }

        self.parse_body();
        Ok(self)
    }

    fn read_file_content(&mut self, file: String) {
        self.req_data = fs::read_to_string(file).expect("not able to read the file");
        let parts = self.req_data.split("\n");
        let content: Vec<&str> = parts.collect();
        if content.len() < 5 {
            println!("invalid request data: {:?}", content);
            process::exit(1);
        }
        self.content = &content[..];
    }

    fn parse_method_and_url(&self) -> Result<(String, String), &'a str> {
        let method_url = self.content.get(0);

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

    fn parse_headers(&self) -> Result<HashMap<&'a str, &'a str>, &'a str> {
        let mut res: HashMap<&'a str, &'a str> = HashMap::new();
        for kv in self.content {
            if kv.len() == 0 {
                break;
            }

            let kv_tmp: Vec<&str> = kv.split(":").collect();
            if kv_tmp.len() != 2 {
                return Err("invlid header");
            }

            res.insert(kv_tmp.get(0).unwrap().trim(), kv_tmp.get(1).unwrap().trim());
        }

        Ok(res)
    }

    fn parse_body(&self) {
        if 2 + self.headers.len() >= self.content.len() {
            self.body = "";
            return;
        }

        let body_content = &self.content[2 + self.headers.len()..];
        self.body = body_content.join("").trim();
    }

    fn send(&self) -> Result<(), &'a str> {
        let client = Client::new();

        let mut hdrs = reqwest::header::HeaderMap::new();
        for (key, val) in self.headers.iter() {
            hdrs.insert(&key[..], val.parse().unwrap());
        }

        let url = self.url.clone();
        let body = self.body.clone();

        let send_with_body = |builder: reqwest::blocking::RequestBuilder| {
            let resp = builder.headers(hdrs).body(body).send();
            println!("resp: {:?}", resp);
        };

        match self.method {
            GET => {
                let builder = client.get(url);
                send_with_body(builder);
                return Ok(());
            }
            PUT => {
                let builder = client.put(url);
                send_with_body(builder);
                return Ok(());
            }
            DELETE => {
                let builder = client.delete(url);
                send_with_body(builder);
                return Ok(());
            }
            POST => {
                let builder = client.post(url);
                send_with_body(builder);
                return Ok(());
            }
            _ => {
                return Err("invalid request method");
            }
        }
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
    let mut request = Request::new();
    let req = request.from_file(*file);
    match req {
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
        Ok(v) => {
            let resp = req.unwrap().send();
            println!("send request result: {:?}", resp);
        }
    }
}
