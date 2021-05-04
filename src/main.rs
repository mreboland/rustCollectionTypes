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


    // VECTORS

    // Just like with arrays, you can use vectors with the type Vec<T> to store multiple values of the same type. Unlike arrays, vectors can grow or shrink at any time. This capability is implied in their size not being known at compile time, so Rust can't prevent you from accessing an invalid position in your vector.
    // When we write Vec<T>, what we're indicating is a Vec type composed of some type T. The nameT is conventionally used as a type name for a type we don't yet know. When we actually create vectors, they'll have concrete types like Vec<u32> or Vec<String>.

    // Using the vec! macro to initialize a vector
    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers); // prints "[1, 2, 3]"

    // The vec! macro also accepts the same syntax as the array constructor
    let ten_zeroes = vec![0; 10];
    println!("Ten zeroes: {:?}", ten_zeroes); // prints [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // The {:?} is used whenever we want to print something for debugging reasons, whereas {} is used for displaying info to an end user. Because Rust doesn't know how to represent a vector of integers to end users, using the {} would result in a compilation error.

    // Vectors can also be created by using the Vec::new() method. We can push values onto the end of a vector, which will grow the vector as needed.
    let mut v = Vec::new(); // creates an empty vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v); // prints [5, 6, 7, 8]

    // We can pop (remove) in much the same way
    let mut y = vec![1, 2];
    let two = y.pop();
    println!("{:?}", two);

    // Vectors also support indexing
    let mut x = vec![1, 2, 3];
    let three = x[2];
    x[1] = x[1] + 5;
    println!("{}", x[1]);

    // Like arrays you can access an element in a vector that doesn't exist
    // let mut x = vec![1, 2, 3];
    // let does_not_exist = v[100];
    // Doing the above will cause the the compiler to 'panic' and abort
    // We can safely access a vector without causing our program to panic by used the Vec::get method which never panics.
}
