use std::thread;

pub fn new_child(name : String){
  let child = thread::spawn(move || {
    for x in 0..5{
      println!("{} : {}",name,x)
    }
  });
}

pub fn new_child2(){
  thread::Builder::new().name("child1".to_string()).spawn(move || {
    println!("Hello, child2!");
  });
}

pub fn workers(max : i32){
  let mut list = vec![];
  for i in 0..max{
    list.push(thread::spawn(move || {
      for x in 0..5{
        println!("{} : {}",i,x)
      }
    }))
  }
  for child in list{
    child.join();
  }
}