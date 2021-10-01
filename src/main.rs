fn main() {
    for j in 1..=15 {
        for i in 0..8 {
            let output =    if j % 15 == 0 {
                                format!("FizzBuzz")
                            } else if j % 5 == 0 {
                                format!("Buzz")
                            } else if j % 3 == 0 {
                                format!("Fizz")
                            } else {
                                format!("{}", i * 15 + j)
                            };

            print!("{:<10}", output);
        }
        println!("");
    }
}
