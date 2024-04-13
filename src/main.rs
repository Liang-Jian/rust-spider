use core::num;
use std::sync::mpsc;
use std::{fs::Permissions, slice};

use sha2::digest::consts::P5;


// fn arg_demo(x:i32)
// {
//     println!("this test {}", x);
// }

// fn return_demo() -> i32
// {
//     5
// }

// fn plus_one(x:i32) -> i32
// {
//     x + 1 
// }

// fn main() 
// {
//     // let y = 6 ;
//     // let x = 32 ;
//     // arg_demo(x);
//     let y = {
//         let x =  3 ;
//         x + 1
//     };

//     println!("The value of y is {}", y);
//     let y = return_demo();
//     println!("This file is {}", y );
//     plus_one(y);

// }

// fn main() {
//     let number = 3 ;
//     if number % 4 == 0 {
//         println!("number is divisionable by 4 ");
//     } else if number % 3 == 0 {
//         println!("number is devisionable by 3 ");
//     } else if number % 2 == 0 {
//         println!("number is devisionable by 2 ");
//     } else {
//         println!("number is not divisiable by. 4 3 or 2 ");
//     }
// }
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 { 
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);
// }

// #[derive(Debug)]
// struct Rect {
//     width:i32,
//     hight:i32,

// }

// // impl Rect {
// //     fn area(width:i32, hight:i32) -> i32
// //     {
// //         width * hight
// //     }
// // }
// impl Rect {
//     fn new(width:i32, hight:i32) -> self {
//         Rect {
//             width,
//             hight
//         }
//     }
//     fn area
// }

// fn main()
// {
//     let demo:Rect = Rect{ width:3, hight:4};
//     let result = Rect::area(4, 3);
//     println!("{}", result);
// }

// struct  Person
// {
//     name:String,
//     age:i32
// }

// impl Person 
// {
//     pub fn new(name:String ,age :i32) -> Self(Person{name, age});    
// }
// fn main()
// {
//     let demo = Person::new(name, age)

// }

// fn main()
// {
//     let dog = "Dog";

//     let cat = "cat";

//     let dog = String::from("fuck");

//     let dog = "dog".to_string();

//     let mut dog   = format!("{}", "dick");

//     dog.push('i');
//     dog.push_str("fuck");
//     println!("{}", dog);

//     let mut kebian = Vec::new();
//     kebian.push("fuck");
//     println!(":?", kebian);
// }


// fn main()
// {
//     let mut count = 0 ;
//     'icon:loop {
//         print!("loop {}", count);
//         let mut remain = 10 ;

//         loop {
         
//         if remain == 1 {
//             break;
//         }

//         if count == 2 {
//             break 'icon;
//         }
//         remain -= 1;
       
//         }
//         count += 1;
//     }
//     println!("ENd count = {}", count);
// }

// fn main()
// {
//     let mut count = 10 ;
//     let result = loop {
//         count += 1 ;

//         if count == 20 {
//             break count;
//         }
//     };
//     print!("the result is ={}",  count);
// }


// fn main()
// {
//     let mut number = 33 ;
//     while number != 0  {
//         println!("number is = {}", number);
//         number -= 1;
//     }
//     print!("give up");
// }


// fn main()
// {
//     let a = [22, 33, 44, 66, 77];
//     let mut index = 0 ;
//     while index < 5 {
//         println!("this result = {}", a[index]);
//         index += 1;
    
//     }

// }

// fn main()
// {
//     let pt = [10,20,30,40,50];
//     for element in pt{
//         println!("the value is :{}", element);
    
//     }

// }

// fn main()
// {
//     for number  in (1..4).rev(){
//         println!("RECV it{}", number);
//     }
//     print!("LIFE OFF");

// }

// fn main()
// {
//     let mut  s = String::from("hello");
//     s.push_str("-fuckme");
//     println!("{}",s);
// }

// fn main()
// {
//     let s2 = String::from("fuck");
//     let s1 = s2.clone();
//     println!("{}",s2);
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }

// fn ownship()
// {


// }

// fn main()
// {
//     let s = String::from("Fcukme");
//     let len = s.len();
//     let slice = &s[..len];
//     println!("{}",slice);
   
// }


// fn main()
// {
//     let stra = String::from("fuck");
//     let len = stra.len();
//     let slice = &stra[..len];
// }

// struct User
// {
//     active:bool,
//     usner:String,
//     age:i32
// }

// fn main()
// {
//     let mut  use1 = User
//     {
//         active : true,
//         usner: String::from("Fuck"),
//         age : 3
//     };
//     use1.usner = String::from("dick");
//     println!("{}", use1.usner);
// }

// struct User
// {
//     isTrue:bool,
//     age:i32,
//     name:String,
// }
// fn build_user(age:i32, name:String) -> User
// {
//     User
//     {
//         age: age,
//         isTrue: false,
//         name: name,
//     }
// }
// fn main()
// {

// }

// fn p3(){
//     println!("print3"); 
// }

// fn p5(){
//     println!("print5"); 
// }
// fn show()
// {
//     let dice_price = 9;
//     match dice_price
    
//      {
//         3 => p3(),
//         5 => P5(),
//         other =>
//     }
// }
// let some_u8_value = Some(0u8);
// match some_u8_value {
//     Some(3) => println!("three"),
//     _ => (),
// }

// mod front_of_house
// {
//     mod hosting{
//         fn add_to_waiting(){}
//         fn seat_to_table(){}
//     }
//     mod serving{
//         fn take_order(){}
//         fn server_order(){}
//         fn take_paymeng(){}
    
//     }
// }


use std::{rc, thread};
use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main()
// {
//     thread::spawn(||{
//     for i in 1..10{
//         println!("hi number {} from the. spawned thread::", i);
//         thread::sleep(Duration::from_secs(1));
//     }
//     });
//     for i in 1..5{
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     )

//     }
// }
// use std::thread;
// use std::sync::mpsc;
// fn main()
// {
//     let (tx, rc) = mpsc::channel();
//     thread::spawn(||{
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//     let received = rx.recv().unwarp();
//     println!("Got {}", received);
// }
// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }



// fn main()
// {
//     let (tx ,rx) = mpsc::channel();
//     thread::spawn(move||{
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     }
//     );
//     let recevied = rx.recv().unwrap();
//     println!("God:{}",recevied);
// }

fn main()
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        
        ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));

        }
    });
    for receved in rx {
        println!("god {}", receved );
    }

}