fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    v.push(6);
    println!("{:?}", v);

    let elem = &v[3];
    println!("{}", elem);

    let elem = v.get(10);
    
    match elem {
        Some(x) => println!("{}", x),
        None => println!("Invalid element.")
    }

    let s = "initial contents".to_string();
    println!("{}", s);
    
    let mut s1 = String::from("foo");
    let s2 = "bar".to_string();
    s1.push_str(s2.as_str());
    println!("{}, {}", s1, s2);

    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let c = &s[0..3];
    println!("{}", c);

    for c in s.chars() {
        println!("{}", c);
    }

    for b in s.bytes() {
        println!("{}", b);
    }
}
