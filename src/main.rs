use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, TcpListener};

#[derive(Debug)]
struct Request {
    // TODO: should be str ?
    path: String,
    host: String,
    user_agent: String,
    accept: Vec<String>
}

fn handle_client(stream: &TcpStream) -> Option<Request> {
    let buf = BufReader::new(stream);
    let mut req = Request {
        path: "".to_string(),
        host: "".to_string(),
        user_agent: "".to_string(),
        accept: Vec::new(),
    };

    for line in buf.lines() {
        let l = line.unwrap();
        println!("{}", l);

        if l == "" {
            break
        }

        let splits: Vec<&str> = l.split(' ').collect();

        match splits[0] {
            "GET" => {
                if splits[2] != "HTTP/1.1" {
                    return None
                }
                req.path = String::from(splits[1])
            },
            "Host:" => req.host = splits[1].to_string(),
            "Accept:" => req.accept = splits[1].split(',').map(|s| s.to_string()).collect(),
            "User-Agent:" => req.user_agent = splits[1].to_string(),  // splits.drain(..1).collect().connect(" "),
            _ => println!("Unhandled header {}", splits[0]),
        };
    }
    return Some(req);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:0").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let req = handle_client(&s);

        println!("{:?}", req);

        match req {
            None => s.write(b"505").unwrap(),
            Some(_r) => s.write(b"HTTP/1.1 200 SUCCESS\n\n<html><body>HELLO FROM RUST</body></html>\n").unwrap(),
        };
    }
    Ok(())
}
