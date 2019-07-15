use std::collections::HashMap;

fn main() {
    {
        let mut v = Vec::new();

        v.push(10);

        for i in 1..10 {
            v.push(i * 2);
        }

        println!("{:?}", v);

        for e in &v {
            println!("{}", e);
        }
    }

    {
        let unicode = String::from("あいうえお");

        println!("length {}", unicode.len());

        for (i, c) in unicode.chars().enumerate() {
            println!("{}: {}", i, c);
        }
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        println!("{:?}", scores);

        let team_name = String::from("Blue");
        if let Some(&v) = scores.get(&team_name) {
            println!("score: {}", v);
        }
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
