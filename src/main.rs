fn main() {
    //let message = "Name: Matt, Height: ";
    //message.clear();
    let proceed = true;
    if proceed {
    println!("Proceeding");
    } else {
        println!("Not Proceeding")
    }

    let height = 75;
    if height > 70 {
        println!("Tall")
    } else if height > 64 {
        println!("Average")
    }else {
        println!("Short")
    }
}
