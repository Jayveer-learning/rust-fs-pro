fn main(){
    // basic numric and string
    let x: i8 = 120;
    println!("{}", x);
    let y: u8 = 121;
    println!("{}", y);
    let z: f64   = 256.00;
    println!("{}", z);

    // string
    let is_name : &str = "Jay veer";
    println!("{}", is_name);

    // for loop
    let mut num : i32 = 0;
    for i_value in 0..100{
        num = num + 10;
        println!("{}", num)
    }

    // conditional statement
    let is_male: bool = true;
    let is_prove: bool = false;

    if is_male{
        println!("Yes He is man");
    } else if  !! is_prove {
        println!("No he is not a man");
    } else {
        println!("I think he was girl")   
    }
}