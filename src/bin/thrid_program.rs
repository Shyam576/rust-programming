use std::io;

fn main(){
    println!("Welcome to Temperature Converter!");
    println!("Choose conversion type: ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    
    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option : u32 = match option.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invaid choice! Please enter 1 or 2");
            return;
        }
    };

    if option == 1 {
        println!("Enter the the temperature you want to convert: ");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        let temp : f64 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number entered");
                return;
            }
        };

        let converted_temp = (temp * 9.0 / 5.0 ) + 32.0;
        println!("Temperature in Fahrenheit : {}",converted_temp);
    }else if option == 2 {
       println!("Enter the the temperature you want to convert: ");
       let mut temp = String::new();

       io::stdin().read_line(&mut temp).expect("Failed to read line");
       let temp :f64 = match temp.trim().parse(){
           Ok(num) => num,
           Err(_) => {
               println!("Invalid number entered");
               return;
           }
       };

       let converted_temp = (temp - 32.0) * 5.0 / 9.0;
       println!("Temperature in celsius: {}", converted_temp);
    }else {
        println!("Invalid choice! Please enter 1 or 2");
    }

    println!("Conversion complete. Goodbye")
}
