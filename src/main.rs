use std::thread;

mod web;
mod thre;
mod socket;

static MAXTHREAD : i32 = 10;
  
fn main(){
  socket::bind("127.0.0.1:80".to_string());
 // thre::workers(MAXTHREAD);
  //web::tcp_server(); 
}