use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("satrt game");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
    	println!("please input your guess. 1~100?");
    	let mut guess = String::new();
    	io::stdin().read_line(&mut guess).expect("failed to read line");
    	let guess: u32 = match guess.trim().parse() {
    		Ok(num) => num,
    		Err(_) => continue,
    	};
    	println!("your guessed: {}", guess);
    	match guess.cmp(&secret_number) {
    	Ordering::Less => println!("Too small!"),
    	Ordering::Greater => println!("Too big!"),
    	// Ordering::Equal => println!("You win!"),
    	Ordering::Equal => {
    		println!("you are winer!");
    		break;
    	}
    }
    
    }
    // println!("the secret number is {}", secret_number);
    // println!("your guessed:{}", guess);
}
