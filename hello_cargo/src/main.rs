use std::io;

fn farenheit_to_celsisus(x :f32, y : String){
    if y == "Farenheit"{
        let temp  = (x * (9.0/5.0)) + 32.0;
        println!("The temperature in farenheit is : {temp}");
    } else if y == "Celsius"{
        let temp = (x - 32.0) * (5.0/9.0);
        println!("The temperature in celsius is {temp}");
    }else{
        println!("No conversion available at this time");
    }
}
fn main(){
    
    println!("please type the temperature you want to convert");
    let mut temp : f32 = 0.0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to get a float");
    temp = input.trim().parse().expect("Not a valid float");
    println!("Type the system you want to convert to either Farenheit or Celsius");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the line");
    /**
     * For user input
     *  */    
     /**
      * loop{
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failure");
            if n.trim() == "quit" {
                break;
            }
            let n : u32 - match n.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
     }
     then you can do fibonacci of n
      */
     
    //farenheit_to_celsisus(temp, choice);
   println!("{}",fibonacci(5));
}
fn fibonacci(x : i32) -> i32{
    match x{
        0 => 1,
        1 => 1,
        _ => fibonacci(x-1) + fibonacci(x-2),
    }
}