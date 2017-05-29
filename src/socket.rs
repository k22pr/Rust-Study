use std::net::{TcpListener, TcpStream};

pub fn bind(addr : String){
  let listener = TcpListener::bind(addr).unwrap();
  
  fn handle_client(stream : TcpStream){
    println!("handle_client");
  }
  
  for stream in listener.incoming(){
    match stream{
      Ok(stream) => {
        handle_client(stream)
      }
      Err(e) => println!("error : {}",e)
    }
  }
}