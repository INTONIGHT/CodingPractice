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
fn main(){
    farenheit_to_celsisus(70,"Celsius");
}