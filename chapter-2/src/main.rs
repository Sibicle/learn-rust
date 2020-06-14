use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("welcome to the game.");

  let secret = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("you must choose:");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("das bad");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("try again");
        continue;
      }
    };

    println!("you say {}", guess);

    match guess.cmp(&secret) {
      Ordering::Less => println!("I say try larger"),
      Ordering::Greater => println!("I say try smaller"),
      Ordering::Equal => {
          println!("I say yes.");
          break;
      }
    }
  }
}
