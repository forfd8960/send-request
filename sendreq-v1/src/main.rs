use reqwest;
use reqwest::blocking::Client;
use std::{collections::HashMap, env, fs, process};

#[derive(Debug)]
struct Content<'s>(&'s [&'s str]);

#[derive(Debug)]
// s is longer than c
struct Request<'c, 's: 'c> {
    method: &'s str,
    url: &'s str,
    headers: HashMap<&'s str, &'s str>,
    body: &'s [&'s str],
    content: &'c Content<'s>,
}

const GET: &str = "GET";
const PUT: &str = "PUT";
const POST: &str = "POST";
const DELETE: &str = "DELETE";

impl<'c, 's> Request<'c, 's> {
    fn new(content: &'c Content<'s>) -> Self {
        Self {
            method: "",
            url: "",
            headers: HashMap::new(),
            body: &[],
            content: content,
        }
    }
    /*

    the request format is:
    method url

    header1: value1
    header2: value2

    {body}
    */
    fn from_content(&mut self) -> Result<&Self, &'c str> {
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

    fn parse_method_and_url(&self) -> Result<(&'s str, &'s str), &'c str> {
        let method_url = self.content.0.get(0);

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
                Ok((method.trim(), url.trim()))
            }
        }
    }

    fn parse_headers(&mut self) -> Result<HashMap<&'s str, &'s str>, &'c str> {
        let mut res: HashMap<&'s str, &'s str> = HashMap::new();
        for kv in self.content.0.iter() {
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

    fn parse_body(&mut self) {
        let length = 2 + self.headers.len();
        if length >= self.content.0.len() {
            self.body = &[];
            return;
        }

        self.body = &self.content.0[length..];
    }

    fn send(&self) -> Result<(), &'c str> {
        let client = Client::new();

        let mut hdrs = reqwest::header::HeaderMap::new();

        let headers = self.headers.clone();
        for (key, val) in headers.iter() {
            // hdrs.insert(&key[..], val.parse().unwrap());
        }

        let url = self.url.clone();
        let body = self.body.clone().to_owned().join("");

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

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        println!("bad arguments");
        process::exit(1);
    }

    let req_file = args.get(0).unwrap();
    let data = read_file_content(req_file.as_str())?;

    let parts = data.split("\n");
    let content: Vec<&str> = parts.collect();
    let c = Content(&content[..]);

    let mut request = Request::new(&c);
    let req = request.from_content();
    match req {
        Err(e) => {
            println!("build request err: {}", e);
            process::exit(1);
        }
        Ok(v) => {
            let resp = v.send();
            println!("send request result: {:?}", resp);
            Ok(())
        }
    }
}

fn read_file_content(file: &str) -> Result<String, &'static str> {
    match fs::read_to_string(file) {
        Err(e) => {
            println!("file err: {}", e);
            return Err("unabled to open file");
        }
        Ok(v) => return Ok(v),
    }
}
