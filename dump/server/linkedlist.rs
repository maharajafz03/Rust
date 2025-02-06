fn main() {
    // Declare an array with a fixed size and type
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // Access elements using their index
    println!("First number: {}", numbers[0]);
    println!("Second number: {}", numbers[1]);

    // Iterate through the array using a `for` loop
    println!("Iterating through the array:");
    for num in numbers.iter() {
        println!("{}", num);
    }

    // Get the length of the array
    println!("The length of the array is: {}", numbers.len());

    // Declare an array with the same value for all elements
    let repeated_array = [1; 5]; // An array of 5 elements, all set to 1
    println!("Repeated array: {:?}", repeated_array);
}
