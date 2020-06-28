use std::env;
use std::fs;
use std::process;
use serde_json::{Result, Value};

extern crate tiny_http;
use tiny_http::{Server, Response};


fn parse_json(input_content: &str) -> Result<Value> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(input_content)?;
    Ok(v)
}


fn main() -> Result<()>{

    let args : Vec<String> = env::args().collect();

    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("damn");

    let v: Value = parse_json(&content).expect("damn");
    
    let map= match v {
        Value::Object(map) => map,
        _ => process::exit(0)
    };


    let server = Server::http("127.0.0.1:1337").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let url = request.url().to_string();
        let route: Vec<&str> = url.split('/').collect();

        if route.len() == 0 {
            continue
        };

        let values = map.get(route[1]);

        let mut response = match values {
            Some(v) => Response::from_string(format!("{}",v)),
            None => Response::from_string("Not Found")
        };

        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();

        response.add_header(header);

        request.respond(response);
    }

    Ok(())
}