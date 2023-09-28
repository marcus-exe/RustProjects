use std::io;

fn main() {
    temperature_converter();
}

fn fahrenheight_to_celsius(fahrenheight : f32) -> f32{
    (fahrenheight - 32.0) * 5.0 / 9.0
}
fn celsius_to_fahrenheight(celsius : f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0 
}
fn temperature_converter(){
    
    //loop as long as they don´t insert a valid option
    let mut choice = loop {
        println!("Select your choice of conversion:");
        println!("From Celsius to Fahrenheight (1)");
        println!("From Fahrenheight to Celsius (2)");
       
        
        let mut choice = String::new();
        //input and error handling
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //if guess is valid, break 
        if choice == 1 || choice == 2 {
            break choice;
        }
        
    };

    if choice == 1 {
        println!("So, you´ve chosen Celsius to Fahrenheight conversion");
        println!("Input your temperature");
        let mut temperature = loop {

            

        };

        celsius_to_fahrenheight(temperature);        
    }


}

