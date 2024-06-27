fn main() {

    // let mut r1: String = String::from("Hello World!");
    // println!("{r1}");
    // r1.push_str("!!");
    // println!("{r1}");

    // let r2 = r1.clone();
    // println!("{r2}");
    // while r2 == r1 {
    //     println!("Break");
    //     break;
    // }
    
    // let s = String::from("Hello World");
    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         println!("Found it: {}", i);
    //     }
    // }
    let int = crate::module_1::module_2::get_one();
    println!("{int}");
}

mod module_1 {
    pub mod module_2 {
        pub fn get_one () -> i32 {
            1
        }
    }
}


