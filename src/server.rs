use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr : String
  }
  
impl Server {
    pub fn new(addr: String) -> Self {
      Self { addr }
    }
  
    pub fn run(self) {
      println!("Listening on {}", self.addr);

      let listener = TcpListener::bind(&self.addr).unwrap();

      loop {
       
        match listerner.accept() {
          ok((mut stream, _)) => {
            println!("OK");
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
              ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              },
              Err(e) => println!("Failed to read from connection : {}", e);
            }
          },
          Err(e) => println!("Failed to established a connection: {}", e);
        }
        
        
        // let res = listerner.accept();  
        // if res.is_err() {
        //   continue;
        // }
        // let stream = res.unwrap();
        
      }
      
    }
  }
