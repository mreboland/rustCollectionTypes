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
    let _three = x[2];
    x[1] = x[1] + 5;
    println!("{}", x[1]);

    // Like arrays you can access an element in a vector that doesn't exist
    // let mut x = vec![1, 2, 3];
    // let does_not_exist = v[100];
    // Doing the above will cause the the compiler to 'panic' and abort
    // We can safely access a vector without causing our program to panic by used the Vec::get method which never panics.

    // HASH MAPS

    // The type HashMap<K, V> stores a mapping of keys of some type K to values of some type V. Where vectors store values by an integer index, hash maps store values by key.
    // Like vectors, hash maps are growable, store the data in the heap, and access to its items are checked at run time.

    // In the following example, we're keeping track of a personal book review system. The keys are the book names, and the values are the reviews made by one specific user.

    // You can create an empty hash map by using the HashMap::new method and then adding elements with the HashMap::insert method.

    // Here we import HashMap from the collections portion of the standard library to bring its name into scope. This is similar to what other programming languages call an import.
    use std::collections::HashMap;

    let mut book_reviews: HashMap<String, String> = HashMap::new();

    // Review some books
    book_reviews.insert(
        // .to_string() methods transforms a string literal (&str) value into String. This method is useful when we want our hash map to "own" the values it holds, instead of being a collection of references (pointers).
        "Adventures of Huckleberry Finn".to_string(),
        "My favourite book.".to_string(),
    );

    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );

    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );

    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Querying our hash map
    if !book_reviews.contains_key("Les Miserables") {
        println!("we've got {} reviews, but Les Mis ain't one", book_reviews.len());
    }

    // Hash maps can use references to query for existing entries. This means that even if our hash map is of type HashMap<String, String>, we can use the &str or &String types to look up its keys.

    // Like with vectors, looking for a nonexistent key causes the program to panic.

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // But searching for a nonexisting key will cause a panic
    // println!("Review for Herman: {}", book_reviews["Moby Dick"]);  // panics!

    // Hash maps also have the .get() method for safely querying their content without causing any panic.

    // We can remove entries from a hash map by using the .remove() method.

    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);

    indexing_tuple(); // no errors, works
    indexing_array(); // no errors, works
}


fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // TODO - Correct the syntax
    // let second = todo!("Replace with the tuple indexing syntax");
    // To access a tuple we use dot notation
    let second = numbers.1;

    assert_eq!(
        2, second,
        "This is not the 2nd number in the tuple: {}",
        second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    // TODO - Correct the syntax
    // let letter_d = todo!("Replace with the array indexing syntax");
    // Like all arrays, we access the value using bracket notation.
    let letter_d = characters[3];

    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d
    )
}