fn main() {
    //let message = "Name: Matt, Height: ";
    //message.clear();
    let proceed = true;
    if proceed {
    println!("Proceeding");
    } else {
        println!("Not Proceeding")
    }
    let mut height: i32 = 75;
    // ---- Shadowing height ----//
    height = height - 15;

    let result = if height > 70 {
        println!("Tall")
    } else if height > 64 {
        println!("Average")
    }else {
        println!("Short")
    };

    println!("Result: {:#}", result);

    let health = if height <180 {"good"} else{"unkown"};
    println!("Health: {}", health)
}
