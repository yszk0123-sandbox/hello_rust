#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    {
        println!("v4 {:?}", IpAddr::V4(255, 255, 255, 255));
        println!("v6 {:?}", IpAddr::V6(String::from("::1")));
    }

    {
        let some_number = Some(5);
        let absent_number: Option<i32> = None;

        print_option(&some_number);
        print_option(&absent_number);
    }

    {
        let some_number = Some(5);
        if let Some(3) = some_number {
            println!("some_number is 3");
        }
    }
}

fn print_option<T>(v: &Option<T>)
where
    T: std::fmt::Debug,
{
    match v {
        Some(v) => println!("Some: {:?}", v),
        None => println!("None"),
    }
}
