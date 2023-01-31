
fn main() {

    // HALF OF A SQUARE: 
    // write a program that uses only two output statements, count << "#" and count << "\n", to
    // produce a pattern of hash symbols shaped like half of a perfect 5 x 5 square (or a right
    // triangle)

    let input : u8 = 5;

    let mut tiles : u8 = input;
    let mut res = String::new();

    for _ in 0..input {
        let current_row = (0..tiles).map(|_| "#").collect::<String>();
        res.push_str(&current_row);
        res.push_str("\n");

        tiles = tiles - 1
    }
        
    // println!("1. HALF OF A SQUARE:\n{}", res);

    // end
    
    // A SIDEWAYS TRIANGLE:
    // write a program that uses only two output statements, count << "#" and count << "\n", to
    // produce a pattern of hash symbols shaped like a sideways triangle

    
    let input : u8 = 5;

    let mut tiles : u8 = input;
    let mut res = String::new();

    // reverse

    let reverse = input - 1;
    let mut input_reverse = 1;

    for _ in 0..reverse {
        let current_row = (0..input_reverse).map(|_| "#").collect::<String>();
        res.push_str(&current_row);
        res.push_str("\n");

        input_reverse = input_reverse + 1
    }

    // NORMAL HALF OF A SQUARE 

    for _ in 0..input {
        let current_row = (0..tiles).map(|_| "#").collect::<String>();
        res.push_str(&current_row);
        res.push_str("\n");

        tiles = tiles - 1
    }


    // println!("2. A SIDEWAYS TRIANGLE:\n{}",res)

    // end 
    
    // LUHN CHECKSUM VALIDATION
    // The luhn formula is a widely used system for validating identification numbers. Using the
    // original number, double the value of every other digit. Then add the values of inidividual
    // digits together (if a doubled value now has two digits, add the digits individually). The
    // identification number is valid if the sum is divisible by 10
    //
    // Write a a program that takes an identification number of arbitrary length and determines
    // whether the number is valid under the Luhn formula. The program must process each character
    // befor reading the next one. 
    
    let mut id_number = String::new();


    std::io::stdin().read_line(&mut id_number).unwrap();

    let trimmed = id_number.trim();

    let numbers_as_string = trimmed.split("").collect::<Vec<&str>>();

    let mut id : Vec<u32> = Vec::new();

    for n in numbers_as_string {
        match n.parse::<u32>() {
            Ok(i) => id.push(i),
            Err(..) => println!("this was not an integer: {}", trimmed),
        }    
    }

    fn multiply_numb_in_odd_position(id: Vec<u32>) -> Vec<u32> {    
        let response = id.clone().into_iter().map(|x| {
            let res1 : Option<usize> = id.iter().position(|&s| s == x);
 
            if(res1.unwrap() & 1 != 0) {
                x * 2
            } else {
                x
            }
        }).collect::<Vec<_>>();

        response
    }

    let all_numbers = multiply_numb_in_odd_position(id);

    fn checksum(all_numbers: Vec<u32>) {
        let sum: i32 = all_numbers.iter().sum();
    }

    checksum(all_numbers)
}

