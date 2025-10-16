fn main(){
    let roman_number1: &str = "X";
    let roman_number2: &str = "XI";
    let roman_number3: &str = "XII";
    let roman_number4: &str = "IX";
    
    let roman_number5: &str = "III";
    let roman_number6: &str = "LVIII";
    let roman_number7: &str = "MCMXCIV";
    
    

    

    println!("should be 10 | {} \n", sum_roman_numbers(roman_number1));
    println!("should be 11 | {} \n", sum_roman_numbers(roman_number2));
    println!("should be 12 | {} \n", sum_roman_numbers(roman_number3));
    println!("should be 9 | {} \n", sum_roman_numbers(roman_number4));

    println!("should be 3 | {} \n", sum_roman_numbers(roman_number5));
    println!("should be 58 | {} \n", sum_roman_numbers(roman_number6));
    println!("should be 1994 | {} \n", sum_roman_numbers(roman_number7));
}

fn discover_value_to_char(character: char) -> i32{
    let what_match: i32 = match character{
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    };
    return what_match;
}

fn sum_roman_numbers(roman_number: &str) -> i32{
    let mut last_value: i32 = 0;
    let mut result: i32 = 0;

    for (i, num_str) in roman_number.chars().enumerate() {
        let num_converted = discover_value_to_char(num_str);

        if i == 0 {
            last_value = num_converted;
            result = num_converted;
        }
        
        else{      
         if num_converted > last_value {
                result = result + num_converted - (2 * last_value);
            }
            else {
                result = result + num_converted;
            }
        }
        last_value = num_converted;
    }
    return result;
}