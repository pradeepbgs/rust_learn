use std::thread;

fn main() {
    let mut name = String::from("pradeep");
    name.push_str(" string");
    println!("Hello, world!, {}", name);

    // owner_ship();
    // if_loop(String::from("Pradeep"));
    // struct_use();
    // enum_use();
    // arr();
    // use_vector()
    // use_traits();
    use_threads();
}

fn owner_ship() {
    let name1 = String::from("praddep");
    let name2 = name1;
    println!("{} ", name2);
}

fn if_loop(x: String) {
    if x == "Pradeep" {
        println!("is Pradeep")
    } else {
        println!("Not Pradeep")
    }

    for i in 0..10 {
        println!("index {} ", i)
    }
}

struct User {
    name: String,
    age:i32
}
fn struct_use() {
    let p1 = User{
        name:String::from("Pradde"),
        age:24
    };
    
    println!("{}",p1.age)
}


// ENUM

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Result::Err(String::from("divide by zero"))
    } else {
        Result::Ok(a / b)
    }
}

fn enum_use() {
    match divide(60, 30) {
        Result::Ok(value) => println!("Result: {}, ", value),
        Result::Err(err) => println!("Error {}, ", err),
    };
}

// Array
fn arr(){
    let arr = [1,2,3];
    println!("{:?},",arr)
}

// Dynamic arrays Vectors
fn use_vector(){
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    
    println!("{:?}, ", v)
}

fn use_traits(){
    trait Speak {
        fn speak(&self);
    }
    
    struct Dog;
    
    impl Speak for Dog{
        fn speak(&self) {
            println!("Bhon Bhon");
        }
    }
    
    let d1 = Dog;
    println!("{:?} ", d1.speak())
}


// Threads
fn use_threads(){
    let handle = thread::spawn(||{
       println!("Hello from a thread") 
    });
    handle.join().unwrap();
}