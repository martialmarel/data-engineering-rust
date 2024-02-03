/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

/*
Ask 1: What is a VecDeque in Rust and how is it different from a Vector or a LinkedList?
Answer: VecDeque is a double-ended queue implemented with a growable ring buffer in Rust.
It provides the functionality of a dynamic array (Vec) and a double-ended queue.

Here are the key differences between VecDeque, Vec, and LinkedList:
    1. VecDeque vs Vec: VecDeque allows efficient pushing and popping at both ends of the sequence,
    while Vec only allows efficient pushing and popping at the end of the sequence.
    However, Vec has a slight advantage in terms of memory usage and access to elements by index,
    as it's a contiguous array.

    2. VecDeque vs LinkedList: VecDeque is generally faster than LinkedList for most operations
    because it's cache-friendlier (it stores its elements in a contiguous block of memory).
    LinkedList allows efficient insertion and removal of elements in the middle of the sequence
    if you have a cursor at the right position, but it's slower for random access because it needs
    to traverse the list.


Ask 2: What is the significance of converting VecDeque to a Vector and then back to VecDeque
in the program?
Answer: Converting a VecDeque to a Vec and then back to a VecDeque is not a common operation and
it's usually not necessary. It can be inefficient because each conversion involves reallocating
memory and copying elements.

However, there could be a few reasons why someone might do this:

    1. Sorting: VecDeque does not have a built-in sort method, but Vec does. If you need to sort
    a VecDeque, you could convert it to a Vec, sort it, and then convert it back.

    2. Interoperability: If you're interfacing with a function that only accepts a Vec, you might
    need to convert your VecDeque to a Vec. After the function call, if you need the double-ended
    queue functionality, you might convert it back to a VecDeque.

    3. Contiguous memory: VecDeque is implemented as a ring buffer, so its elements may not be in
    a single, contiguous block of memory. If you need a contiguous slice of memory, you could
    convert it to a Vec.

Ask 3: Why do we push "Pomegranate" to the front of the queue and "Fig" and "Cherry" to the back
of the queue after shuffling?
Answer: The decision to push these specific elements to the front or back of the queue after shuffling could be based
on their importance the need to maintain a certain order.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
