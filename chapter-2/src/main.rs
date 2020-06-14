use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
  println!("welcome to the game.");
  println!("you must find the number");

  let secret = rand::thread_rng().gen_range(1, 101);

  loop {

    let mut guess = String::new();

    print!("you say ");

    io::stdout()
      .flush()
      .expect("das bad");

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

    match guess.cmp(&secret) {
      Ordering::Less => larger(secret, guess),
      Ordering::Greater => smaller(secret, guess),
      Ordering::Equal => {
          println!("I say yes.");
          break;
      }
    }
  }
}

fn larger(secret: u32, guess: u32) {
  distance(secret, guess, true);
}

fn smaller(secret: u32, guess: u32) {
  distance(secret, guess, false);
}

fn distance(secret: u32, guess: u32, dir: bool) {
  let size = if dir { "larger" } else { "smaller" };

  let delta = if dir {
    (secret - guess) / 10
  } else {
    (guess - secret) / 10
  };

  let amount = match delta {
    6..=10 => String::from(format!("much, much, much {}", size)),
    4..=5  => String::from(format!("a lot {}", size)),
    2..=3  => String::from(format!("{}", size)),
    1      => String::from(format!("a little bit {}", size)),
    0      => String::from(format!("a teensy bit {}", size)),
    _      => String::from("something who knows")
  };

  println!("I say try {}", amount);
}
