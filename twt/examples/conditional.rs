fn main() {
    let num1 = (20 as u8);
    let num2: u8 = 20;

    let cond = num1 != num2;

    // compound conditions

    let cond2 = false || !cond;

    // println!("{}",cond);
    // println!("{}",cond2);
    control_flow();
}

fn control_flow() {
    let food = "cookie";

    if food != "cookie" {
        println!("I love cookies.");
    } else if food == "cookie" {
        println!("You have cookies, please give it to me.");
    } else {
        println!("Where are cookies?");
    }
}