fn main(){
let  x = 5;
 let x = x+1;
{
     let x = x*2;
    println!("Value of x in the inner scope is :{x}");
}
println!("The value of x is : {x}");
let z = 2.0;//f64
let w : f32 = 3.0;//f32
let sum = z + w;
let difference = z - w;
let product = z * w;
let quotient = z / w;
let floored = z / w;
let remainder = z % w;
println!("sum is {sum} difference {difference} product is {product} quotient is {quotient} floored {floored} remainder {remainder}");
let f : bool = false;
let tup : (i32,f64,u8) = (500,6.4,1);
let (a,b,c) = tup;
println!("The value of b is {b}");
//decalres an array with specific type and length
let array : [i32;5] = [1,2,3,4,5];
another_function(5);
let lamb = plus_one(90);
println!("Value of function plus one for 90 is {lamb}");
let number = 10;
if number <  5{
    println!("condition was true");
} else{
    println!("Condition was false");
}
farenheit_to_celsisus(70,"Celsius");
}

fn another_function(x: i32){
let y = {
    //this cant be a semicolon as it is an expression not a statement
    x+1
};
println!("The value of y is {y}");
}

fn plus_one(x: i32) -> i32{
    //if you were to add a semicolon after this it will error out.
    x+1
}
/**
 * Multi line comment
 * which is easier to use than //
 */
fn control_flow(){
    let condition = true; 
    let number = if condition {5} else {6};
    println!("The number is {number}");
}

fn loops(){
    let mut counter =0;
    let result = loop{
        counter +=1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn multiple_loops(){
    let mut count =0;
    //to nest loops must have single quote
    'counting_up : loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("End count = {count}");
}

fn while_loop(){
    let mut num = 3;
    while number !=0{
        println!("{numnber}");
        number -= 1;
    }
    println!("Liftoff");
    let a =[10,20,30,40,50];
    let mut index = 0;
    while index<5 {
        println!("value is {}" a[index]);
        index += 1;
    }
    //for loop
    for element in a {
        println!("the value is {element}");
    }
    //this will loop from 4 to 1 the rev reverses the 1 to 4 and the .. is for a range.
    for numbers in (1..4).rev(){
        println!("{numbers}");
    }
    println!("Liftoff");
}
fn farenheit_to_celsisus(x :i32, y : String){
    if y == "Farenheit"{
        let temp = x * (9/5) + 32;
        println!("The temperature in farenheit is : {temp}");
    } else if y == "Celsius"{
        let temp = (x - 32) * (5/9);
        println!("The temperature in celsius is {temp}");
    }else{
        println!("No conversion available at this time");
    }
}