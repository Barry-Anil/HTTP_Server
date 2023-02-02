//HTTP/1.1 

// 1. L7 Protocol/ Application layer protocol.
// 2. Sent over TCP.
// 3. Message based.

// Struts : 
// Associated Functions
// Method

// ENUMS 
// 1. Option
// 2. Result

// Match 
// match "abcd" {
//  "abcd" => println!("yes"),
//   "a" | "b" => {},
//    _ => {}
//}



/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;


fn main() {

    // let get = Method::GET("hello".to_string());
    // let post  = Method::POST;
    // let put = Method::PUT;
    // let delete = Method::DELETE(100);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

