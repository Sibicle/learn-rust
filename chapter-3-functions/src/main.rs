fn main() {
    println!("Hello, world!");

    basic();
    params(43);
    str_params("hello".to_string(), 43);
    array(4, 2);
    // array(8); // out of bounds
    state();
}

fn basic() {
    println!("Another function.");
}

fn params(x: i32) {
    println!("The value of x is: {}", x);
}

fn str_params(x: String, y: i32) {
    println!("{}, {}", x, y);
}

fn array(n: usize, m: usize) {
    let a: [usize; 7] = [1, 7, 2, 6, 18, 5, 70];

    println!("The value at position {} is: {}", n, a[n]);

    let b = [4, 5, 7, 8];

    println!("The value {} is at position {} of {}", b[m], m, b.len());
}

fn state() {
    println!("{}", express(5));
}

fn express(num: usize) -> String {
    let mut numm = num;

    for _ in 1..num {
        numm = numm + num;
    }

    format!("{} meows", numm)
}
