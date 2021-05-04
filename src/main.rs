fn main() {
    
    // ARRAYS

    // An array is a collection of objects of the same type, which are stored sequentially in memory. Arrays are created by using brackets []. Their size, which is known at compile time, is part of their type signature [T; size], where T is the type of the values in the array and size is a nonnegative integer checked at compile time.

    // Simplified, arrays have a fixed length and every element must be of the same type

    // An array can be defined in two ways:

    // A comma-separated list inside brackets
    // The initial value, followed by a semicolon, and then the length of the array in brackets

    // A comma-separated list inside of brackets
    let _weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    // Initialize an array of 512 elements where every element is a zero
    let _byte_buffer = [0_u8; 512];

    // Arrays are useful when you want your data allocated on the stack rather than the heap. They're also useful when you want to ensure you always have a fixed number of elements.
    // http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html

    // We can access elements by using indexing which starts at 0
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}", letters[0]);  // prints 'a'
    println!("second element of the array: {}", letters[1]); // prints 'b'
    // If we try to access an element of an array that doesn't exist, we get a compile error.
    // println!("invalid array access: {}", letters[99]);  // our `letters` array has only 7 elements


    

}
