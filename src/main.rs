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
 

fn main() {
    // Yslovnie operatori
    let number = 10; 

    if number > 5 {
        println!("Number bigger than 10")
    }
} 
