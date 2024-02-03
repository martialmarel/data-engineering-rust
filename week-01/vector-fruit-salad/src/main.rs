/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

/*
Ask 1 : What is a Vector in Rust and how is it different from arrays?
Answer : A Vector in Rust, often referred to as Vec<T>, is a resizable array type.
Unlike arrays, vectors in Rust can grow or shrink in size at runtime.
Here are some key differences between vectors and arrays in Rust:
    - Size: The size of an array is fixed at compile time, while a vector can grow or shrink at runtime.
    - Flexibility: Vectors are more flexible than arrays. You can easily add or remove elements from a vector,
    but not from an array.
    - Storage: Arrays are stored on the stack, while vectors are stored on the heap.

Ask 2 : What is the use of the SliceRandom trait from the rand crate in the program?
Answer : The SliceRandom trait from the rand crate in Rust provides random sampling methods for slices.
It's typically used when you want to shuffle a collection or select random elements from it.

Ask 3 : Why is enumerate() method used while printing the fruits? What functionality does it provide?
Answer : The enumerate() method in Rust is used when you want to iterate over an iterable (like an array
or a vector) and you also want to keep track of the index of the current item.

When you call enumerate() on an iterable, it returns an iterator that yields pairs of (index, element).
The index is a count of the iterations, and element is the value returned by the iterable.

*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;

fn main() {
    // read stdin and create a vector of fruit
    let mut fruit = vec![];
    loop {
        println!("Enter a fruit (or press Enter to finish): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        fruit.push(input.to_owned());
    }

    let mut rng = thread_rng();
    // Scramble (shuffle) the fruit
    fruit.shuffle(&mut rng);

    // Choose a random fruit to remove
    let random_fruit = fruit.choose(&mut rng);
    println!("A random fruit choosen :{:?}", random_fruit);

    // Print out the fruit salad
    println!("Fruit Salad ({}): ", fruit.len());
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("({}) {}, ", i, item);
        } else {
            println!("({}) {}", i, item);
        }
    }
}
