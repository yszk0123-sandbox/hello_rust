fn main() {
    println!("[Loop]");
    do_loop();

    println!("[While]");
    do_while();

    println!("[For]");
    do_for();
}

fn do_loop() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count: {}", count);
        if count >= 10 {
            break;
        }
    }
}

fn do_while() {
    let mut count = 0;

    while count <= 10 {
        println!("count: {}", count);
        count += 1;
    }
}

fn do_for() {
    let items = [10, 20, 30];

    for item in items.iter() {
        println!("item: {}", item);
    }
}
