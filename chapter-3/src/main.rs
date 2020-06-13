fn main() {
  shadow();
  tup();
  tuptwo();
  // err();
}

fn shadow() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn tup() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of (x, y, z) is: ({}, {}, {})", x, y, z);
}

fn tuptwo() {
    let tup = (500, 6.4, 1);

    println!("The value of tuple(x, y, z) is: ({}, {}, {})", tup.0, tup.1, tup.2);
}

fn err() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
