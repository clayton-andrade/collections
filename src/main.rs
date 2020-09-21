use std::collections::HashMap;

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

    let mut placares = HashMap::<String, u32>::new();
    // let mut placares: HashMap<String, u32> = HashMap::new();
    placares.insert("Azul".to_string(), 10);
    // placares.insert("Amarelo".to_string(), 50);
    placares.entry("Amarelo".to_string()).or_insert(50);
    placares.entry("Azul".to_string()).or_insert(50);
    println!("{:?}", placares);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let name = "Color".to_string();
    let value = "Blue".to_string();

    let mut map = HashMap::new();
    map.insert(name, value);

    println!("{:?}", map);
    // println!("{}, {}", name, value);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);
    match score {
        Some(x) => println!("Score: {}", x),
        None => println!("Not found.")
    }

    for (k, v) in &placares {
        println!("{}: {}", k, v);
    }

    let words = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let letters = "lakahstshfnfksmsmsmsjjjystsgdrndsççççsksjsustegd".to_string();
    let mut map_letters = HashMap::new();

    let mut count;
    for l in letters.chars() {
        count = map_letters.entry(l).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map_letters);
}
