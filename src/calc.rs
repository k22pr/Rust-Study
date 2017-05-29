pub fn plus(a: i32,b: i32) -> i32{
  return a+b;
}

pub fn minus(a: i32, b : i32) -> i32{
  return a-b;
}

pub fn row_pointer(){
  let x = 5;
  let raw = &x as *const i32;
  
  let points_at = unsafe { *raw };
  
  println!("raw points at {}", points_at);
}