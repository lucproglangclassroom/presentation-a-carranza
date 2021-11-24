use std::io;


fn main() {
    let mut vector = Vec::new();
    let mut max = 0;

    while max < 3 {
        println!("Please enter a number: ");
        let mut input = String::new();
        io::stdin().read_line( & mut input); //takes a mutable reference to a string buffer

        let a:u32 = input.trim().parse().unwrap(); //makes sure the input is an int character

        println!("Your input: {}", a);

        vector.push(a); //updates the vector
        println!("{}", vector);
        max += 1;
    }
    let (sum, avrg) = calculate(vector);

    println!("Sum and average, respectively: {:?}", (sum, avrg));

}
fn calculate(a: Vec<u32>) -> (u32, u32){
    let mut sum = 0;
    let mut cap = 0;
    for i in a {
        let i: u32 = i;
        sum += i;
        cap += 1;
    }
    (sum, sum/ cap)
}

