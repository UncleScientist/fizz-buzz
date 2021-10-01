fn main() {
    for j in 1..=15 {
        for i in 0..8 {
            let output = if j % 15 == 0 {
                "FizzBuzz".to_string()
            } else if j % 5 == 0 {
                "Buzz".to_string()
            } else if j % 3 == 0 {
                "Fizz".to_string()
            } else {
                format!("{}", i * 15 + j)
            };

            print!("{:<10}", output);
        }
        println!();
    }
}
