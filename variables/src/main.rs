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
another_function();
}
fn another_function(){
println!("The other function");
}