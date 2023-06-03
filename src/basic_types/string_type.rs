fn main() {
    let string_variable = String::from("127.0.0.1:8080"); // String
    let string_slice = &string_variable[0..9]; // &str (string slice)
    let string_borrow = &string_variable; // &String (borrowed string)
    let string_literal = "1234";

    dbg!(string_variable);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    struct Server {
        address: String,
        port: u16,
    }

    impl Server {
        fn new(address: String, port: u16) -> Self {
            Self { address, port }
        } // explanation about impl
          // every impl block has a hidden self parameter that is passed to every function
          // the self parameter is a reference to the object the method is being called on
          // so, if we call the run method on a Server object, the self parameter will be a reference to that object

        fn run(&self) {
            println!("{}:{}", self.address, self.port);
        }
    }

    let address: String = "127.0.0.1:8080";
    let port: u16 = 8080;
    let server = Server::new(address, port);
    server.run();

    let portSelf: u16 = server.port;
    println!("{}", portSelf);

    struct Request {
        method: String,
        url: String,
        headers: Vec<String>,
        body: Vec<u8>,
        path: String,
    }

    enum Method {
        GET,
        POST,
        PUT,
        DELETE,
    }

    impl Method {
        fn to_string(&self) -> String {
            match self {
                Self::GET => "GET".to_string(),
                Self::POST => "POST".to_string(),
                Self::PUT => "PUT".to_string(),
                Self::DELETE => "DELETE".to_string(),
            }
        }
    }

    impl Request {
        fn new(method: Method, url: String, headers: Vec<String>, body: Vec<u8>) -> Self {
            let path = url.split("/").last().unwrap().to_string();
            Self {
                method: method.to_string(),
                url,
                headers,
                body,
                path,
            }
        }
    }
}
