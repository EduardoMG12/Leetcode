use std::collections::HashSet;

fn main(){
    let first_array = vec![0, 2, 3, 5, 5]; // true
    let second_array = vec![0, 2, 3, 5, 6]; // false    

    println!("First vec: {:?} | return: {}", first_array, contains_duplicate(&first_array));
    println!("Second vec: {:?} | return: {}", second_array, contains_duplicate(&second_array));


}

fn contains_duplicate(array: &Vec<i32>) -> bool{
    let mut showed_numbers = HashSet::new();

    for &number in array.iter(){

        if !showed_numbers.insert(number) {
            return true;
        }
    }
    false
}