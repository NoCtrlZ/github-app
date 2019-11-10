use std::net::TcpListener;

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        println!("{:?}", stream);
    }
}

fn main() {
    let port = ":3000";
    instance_listen(port);
}