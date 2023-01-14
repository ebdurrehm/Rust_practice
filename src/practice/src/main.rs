use std::io;

fn main() {
   const MULTIPLE_BY:f64 = 5 as f64 /9 as f64;
loop {
   let mut degree: String = String::new();
   println!("please enter temperatures in degrees Fahrenheit");
   match io::stdin().read_line(&mut degree) {
      Ok(n)=> {
        println!("{n} bytes received");
        println!("you entered {degree} F");
      },
      Err(_err) => println!("Something went wrong"),
   };

   let degree:i32 = match degree.trim().parse() {
    Ok(result)=> result,
    Err(_) => continue
   };
   let degree_in_celsius:f64 = ((degree - 32) as f64 * MULTIPLE_BY).round(); 
   println!("{} Fahrenheit equals to {} Celsius", degree, degree_in_celsius);
}
}
