use std::io;


fn find_iterations(mut n: i64) -> i16 {
    let mut c = 0;
    loop {
        if n == 1 {
            return c;
        }
        
        if n % 2 == 0 {
            n = n / 2;
        }
        else {
            n = 3 * n + 1;
        }
        
        c += 1;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    return input;
}

fn main() {
    println!("Which of these would you like to find out?\n \
    1. The number of iterations it'll take to get to 1\n \
    2. The highest number of iterations it'll take to get to 1 for a range of numbers"
);
    let input = get_input();
    if input.trim() == "1" {
        println!("Provide a number to calculate the number of iterations for:");
        
        let number = get_input().trim().parse::<i64>().unwrap();
        
        println!("It took {} iterations to get to 1 for the value: {}", find_iterations(number), number)
    }
    else if input.trim() == "2" {
        println!("Please provide a number to calculate the highest number of iterations it'll take to get to 1 for every number within it");
        
        let number = get_input().trim().parse::<i64>().unwrap();
        let mut highest: (i64, i16) = (0, 0);
        for n in 2..number {
            let iterations = find_iterations(n);
            if iterations > highest.1 {
                highest.0 = n;
                highest.1 = iterations;
            }
        }

        println!("The most number of iterations it took to get to 1 was for: {}, with {} iterations.", highest.0, highest.1);
    }
}