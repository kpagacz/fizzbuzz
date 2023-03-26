fn skip(_n: i32) {}

fn print(n: i32) {
    print!("{}", n);
}

fn test(n: i32, divisor: i32, label: &str, current_function: fn(i32) -> ()) -> fn(i32) -> () {
    if n % divisor == 0 {
        print!("{}", label);
        return skip;
    } else {
        return current_function;
    }
}

fn fizzbuzz(n: i32) {
    let mut current_function: fn(i32) -> () = print;
    current_function = test(n, 3, "Fizz", current_function);
    current_function = test(n, 5, "Buzz", current_function);
    current_function(n);
}

fn main() {
    for i in 1..101 {
        fizzbuzz(i);
        println!("");
    }
}
