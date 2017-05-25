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

}