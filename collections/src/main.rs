use std::collections::HashMap;
//How to use hashmaps


fn main() {
    //declaring a vector that is empty.
    //Vec uses Vec<T>
    let mut v : Vec<i32> = Vec::new();
    //using a macro to create it and let rust infer the type.
    let w = vec![1,2,3,4,5];
    //adding elements.
    v.push(5);
    //getting based on index
    let third: &i32 = &w[2];
    println!("The third element is {}",third);
    let third: Option<&i32> = w.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    for i in &w {
        println!("{}",i);
    }
    //if mutable
    let mut x = vec![100,32,57];
    for i in &mut x {
        //this * is for dereferencing not for multipyling.
        *i +=50;
    }
    enum SpreadhseetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadhseetCell::Int(3),
        SpreadhseetCell::Text(String::from("blue")),
        SpreadhseetCell::Float(10.12),
    ];
    hash_maps();
}

fn strings() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let mut s = String::from("foo");
    s.push_str("bar");
    //push is for a singular character otherwise u need push_str
    s.push('l');
    //concating strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    //multiple strings
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    
    let s7 = format!("{}-{}-{}",s4,s5,s6);

    for c in s7.chars() {
        println!("{}", c);
    }
    
}
/**
     * for b in s7.bytes() {
     * println!("{}", b)
     * }
     */
fn hash_maps() {
    let mut scores = HashMap::new();
    //inserting values with value and keys
    scores.insert(String::from("Blue"), 10);
    //checks if a value is present and only inserting if not existing
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score} is the value of score");

    for (key,value) in &scores {
        println!("{} : {}", key, value);
    }
    //or you can do println!("{} is the value of score", score);
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:?}", map);
}