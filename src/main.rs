    // SAMOE PERVOE VIVOD I TYPE DANNIX
      /* fn main() { 
    println!("Hello, world!");

    // Comment odna stroka comment

    /* mnogo strochek comment
    
    
    
     */

// Integer: i8, i16, i32, i64, i128 == eto type dannix prinimaet i otricatelnoe i polojitelnoe
    // u8, u16, u32, u4, u128 == eto type dannix prinimaet tolko polojitelnie chisla

    let num:u8 = 50; // mut eto mutable, izmenyaet peremnyy, mojno menyat
    println!("Result: {}", num);
     let num: i16 = -4500; 
    println!("Result: {}", num);

    let num: u64 = 1000000;
    println!("Result: {}", num);

 
 // Float chislo s plavayshee tochkoi

let num: f32 = 5.453;
println!("Result: {}", num);

let num:f64 = 5.453234;
println!("Result: {}", num);

// Boolean eto type dannix bylevoe znachenie == true ili false

let is_has_car: bool = false;
println!("Result: {}", is_has_car);

// Char type dannix chto mojet soderjat lyboi znak

let sym: char = '%';
println!("Result: {}", sym);

} */
 
 // CONST, TUPLE, ARRAYS - PRINIYATIE DANNIX OT POLZOVATELYA IO   
    /* fn main() {
    // Const, tuple, arrays
    // Constanti eto peremenay kotoroy menyat nelzya
    const USER_MAX_SCORE: u32 = 1_000_000;
    println!("Info: {}", USER_MAX_SCORE);

    // Tuple 
    let mut user_alex: (i32, bool, f64, char) = (42, true, 1.86, 'R');
    user_alex.2 = 2.0;
    println!("Info: {}", user_alex.2);

    //Array
    let mut nums: [i8; 5] = [1, 5, 2, 7, 3];
    nums[0] = 10;
    println!("Info: {}", nums[0])
 }   */

 /* use std::io;

fn main() {
    // User input
let mut num1 = String::new();
let mut num2 = String::new();

println!("Enter num1");
io::stdin().read_line(&mut num1).expect("Fail to read information");

println!("Enter num2");
io::stdin().read_line(&mut num2).expect("Fail to read information");

let data1: i16 = num1.trim().parse().expect("Please enter a valid number");
let data2: u8 = num2.trim().parse().expect("Please enter a valid number");

   println!("Result 1: {}, result 2: {}", data1, data2);
   let mut res: i16 = data1 + data2 as i16;
    println!("Result {}", res);

    res += 1;
    println!("Result {}", res);

    
}  */
 
// YSLOVIE OPERATORI IF, IF ELSE, ELSE

  /* fn main() {
    // Yslovnie operatori
    let number = 5; 
    let is_has_car = true;
    // Operatori && eto i and   || eto ili or 
    // && eto i and   || eto ili or 
    if number >= 5 || is_has_car {
        println!("Number bigger than 10")
    } else if number == 4 {
        println!("Number is 4")
    } else {
        println!("else operator")
    }
}  */

// TERNARNIE OPERATORI CONDITION

  /*  fn main(){
    // Ternarnii operator
    let condition: bool = true;
    let number = if condition { 5 } else { 10 };

    println!("Res: {}", number)
  }  */

  // OPERATOR MATCH


 /* fn main(){
    // Operator match
    let number = 6;

    match number {
    1 => println!("Result 1"),
    2 => println!("Result 2"),
    3 => println!("Result 3"),
    4 => println!("Result 4"),
    5 => println!("Result 5"),
    _ => println!("Else")
    }
    println!("")
} */

// LOOPS RUST FOR IN

 /*
 fn main() {
    // Loops Rust
    for i in 1..=10{ // prosto begaet ot 1 do 10 i pokazivaet
        println!("Number: normal {}", i)
    }
    for i in (1..=10).rev(){ // reverse ny rev() begaet naoborot ot bolshego k menshemy
        println!("Number: reverse {}", i)
    }
    for i in (1..=10).rev().step_by(2){ // step_by pokazivaet pereskok cherez 2 elementa
        println!("Number: reverse + step_by {}", i)
    }
}  */

// LOOPS RUST WHILE

 /* fn main(){
// Loops Rust
let mut number =3;


 while number >0 {
    println!("Result: {}", number);
    number -=1;

    println!("Finish");
     
 }
} */

fn main(){
    let mut number = 0;

    let result = loop {
        number += 10;

        if number >= 50{
        break number;
  } else {
            println!("vse, pizda {}", number)
        }
    };
println!("hop, vot ono {}", result);

}