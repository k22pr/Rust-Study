fn main() {

    //struct study
    struct Point {
        x : i32,
        y : i32
    }

    let mut orgin = Point{x:10,y:20};
    orgin.x = 10;
    println!("orgin = {}, {}",orgin.x, orgin.y);
    match orgin.x {
        1 => println!("one"),
        10 => println!("ten"),
        _ => println!("Unkown"),
    };

    /*
    let tuple : (i32, String) = (5, String::from("five"));
    let (x, _s) = tuple;
    //println!("tuple is {:?}",tuple);
    let tuple = (x,_);
    println!("typle is {}",tuple);
    */
    /*
    let hel = String::from("  hello  ").trim();
    println!("im.. {}",hel);
    */

    // match study
    let x = 1;
    match x{
        ref r => println!("this r,x is : {}, {}",r,x),
    }
    let x = 1;
    match x{
        e @ 1...5 => println!("x is early number {}",e),
        e @ 6...9 => println!("x is late number {}", e),
        _         => println!("x is unknown")
    }

    struct Person{
        name: Option<String>,
    }
    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _                 => {},
    }

    let x = 5;
    match x{
        e @ 1...5 | e @ 8...10 => println!("got a range element {}",e),
        _                     => println!("anything"),
    }

    enum OptionalInt{
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    match x{
        OptionalInt::Value(i) if i > 5 => println!("x is bigger than five!"),
        OptionalInt::Value(..)         => println!("Got an int!"),
        OptionalInt::Missing           => println!("No such luck.")
    }
    //Mix and Match
    
    //String
    let mut greeting ="Hello there.";
    
    let mut s = "Hello".to_string();
    println!("{}",s);
    s.push_str(", world");
    println!("{}",s);
    
    let mut greeting ="Hello there.";
    println!("{}",greeting);
    //greeting.push_str(" yep."); error this.
    
    fn takes_slice(slice : &str){
        println!("Got : {}",slice);
    }
    
    let s3 = "Hello".to_string();
    takes_slice(&s3);
    
    
    // String indexing
    let s4 = "hello";
    //println!("The first letter of s is {}",s4[0]); this error.
    
    let hac = "안녕하세요";
    for b in hac.bytes(){
        print!("{}, ",b);
    }
    println!("");// \n
    
    for c in hac.chars(){
        print!("{}, ",c);
    }
    println!("");
    
    //Slicing
    
    let dog = "hachiko";
    println!("dog : {}",&dog[0..5]);
    
    let dog = "忠犬ハチ公";
    //println!("dog : {}",&dog[0..2]); this error.
    
    let h = "hello ".to_string();
    let w = "world!";
    let hw = h+w;
    println!("hw : {}",hw);
    
    
}