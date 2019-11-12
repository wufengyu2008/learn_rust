fn main() {
    println!("Hello, world!");
    let x = 5;
    // println!("the value of x is: {}", x);
    // x = 6;
    // println!("the value of x is: {}", x);
    // x = 0;
    // println!("the value of x is: {}", x);
    let x = x+1;
    let x = x*2;
    println!("the value of x is: {}", x);
    // const MAX_POINTS: u32=100_000;

    let spaces = "      ";
    // spaces = spaces.len();  不能改变变量类型
    let spaces = spaces.len();
    println!("spaces len is {}", spaces);
}
