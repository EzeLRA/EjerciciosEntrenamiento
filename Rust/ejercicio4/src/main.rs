/*
    Friendly Numbers
    + time limit per test => 1 second
    + memory limit per test => 256 megabytes

For an integer "x", we call another integer "y" friendly if the following condition holds:
                [ y − d(y) = x ]
, where "d(y)"" is the sum of the digits of "y".

For a given integer "x" , determine how many friendly numbers it has.

Input:
    Each test contains multiple test cases. The first line contains the number of test cases t (1≤t≤500). 
    The description of the test cases follows.

    Each test case consists of a single line containing one integer "x" (1≤x≤109).

Output:
    For each test case, output one integer — the answer to the problem.

Example:
    Input:
    3 <= cant of test cases
    numbers to be tested :
    1
    18
    998244360

    Output:
    0
    10
    10
Note:
    The number 1 does not have any friendly numbers.
    
    The number 18 has 10 friendly numbers: These are all the numbers from 20 to 29. 
        For example, 20−d(20)=20−2=18.

    The number 998244360 has 10 friendly numbers:
        998244400
        998244401
        998244402
        998244403
        998244404
        998244405
        998244406
        998244407
        998244408
        998244409
*/

fn digit_sum(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn is_friendly(n: u32,x: u32) -> bool {
    if (n - digit_sum(n)) == x {
        return true;
    }
    false
}

fn num_reader() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let mut x : u32 = 0;
    let mut friendly_numbers : u32 = 0;

    //Verifing if x have friendly numbers
    /*  Note:
         In this point when "x" is not divisible by 9 or not . This part uses a arithmetic "trick" to determine if "x" have friendly numbers or not.
          Considering the residuo of the operation , when:
            - "x" is divisible by 9 => "x" have friendly numbers because "y - d(y)" gave another divisible by 9 number.
            - "x" is not divisible by 9 => "x" does not have friendly numbers
    */
    if (x % 9) == 0 {
        //Reading the number of test cases and the numbers to be tested
        for _ in 0..num_reader() {
            //Reading the number to be tested
            x = num_reader();
            let mut number : u32 = x;

            //Finding the friendly numbers
            /*
                Note:
                 Since "x" is at most "10^9", the number "y" will have approximately 10 digits. 
                 The maximum sum of digits in such a number is "9 * 10 = 90". 
                  This means that "y" will never be more than 90-100 units away from "x". 
                  And don't needs to test millions of numbers for "y"."
                  
            */
            while number <= (x + 100) {
                if is_friendly(number,x) {
                    friendly_numbers += 1;
                }
                number += 1;
            }
            println!("{}", friendly_numbers);
            friendly_numbers = 0;
        }
    } else {
        println!("{}",friendly_numbers);
    }
}
