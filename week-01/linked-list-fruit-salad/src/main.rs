/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

/*
Ask 1: What is a LinkedList in Rust and how is it different from a Vector or a VecDeque?
Answer: In Rust, a LinkedList is a data structure that consists of a sequence of elements, each
linked to the next by a pointer. It allows for efficient insertion and removal of elements from
both ends of the sequence.

On the other hand, a Vector (or Vec in Rust) is a resizable array type. It's good for situations
where you want to push items onto the end of a list and reference them by index, but it can be
expensive to add or remove items in the middle because it requires shifting all subsequent items.

VecDeque is a double-ended queue implemented with a growable ring buffer. It provides efficient
front and back insertion and removal, but unlike LinkedList, it also provides reasonably efficient
indexing.

Here are the key differences:

    1. Memory layout: Vec and VecDeque are contiguous in memory, while LinkedList is not.
    This means that vectors can have better cache performance.

    2. Insertion and removal: LinkedList has constant time insertion and removal at both ends,
    while Vec has constant time insertion and removal at the end only. VecDeque has constant time
    insertion and removal at both ends.

    3. Indexing: Vec and VecDeque support efficient indexing, while LinkedList does not. To access
    the nth element of a LinkedList, you have to iterate from the start or end of the list.

In general, you should default to using Vec in Rust because of its cache friendliness and because
it's often faster in practice than LinkedList, even for operations that are theoretically faster on
linked lists. Use VecDeque when you need fast push/pop on both ends. Use LinkedList only when you
have a specific need for its unique characteristics.


Ask 2: In what situations might you prefer to use a LinkedList over other data structures?
Answer: You might prefer to use a LinkedList in Rust in the following situations:

    1. Frequent insertions and deletions in the middle: If your program frequently inserts or
    removes elements in the middle of the sequence, a LinkedList could be more efficient because
    these operations are O(1), assuming you already have a cursor at the location.
    In contrast, these operations are O(n) in a Vec or VecDeque because they have to shift all
    elements after the insertion or removal point.

    2. Large elements: If the elements are large, it might be more efficient to move around pointers
    to those elements (as in a LinkedList) rather than the elements themselves (as in a Vec or
    VecDeque).

    3. Unknown size at creation: If you don't know the maximum number of elements at the time of
    creation and the number of elements can be quite large, a LinkedList can be a good choice
    because it doesn't need to allocate a large block of memory upfront.

    4. Implementing other data structures: LinkedList is often used as a building block for other
    data structures like a Stack or a Queue.

However, it's worth noting that in many cases, a Vec or VecDeque can be more efficient in Rust due
to better cache locality, even for operations that are theoretically faster on a LinkedList.
Therefore, it's recommended to benchmark your specific use case to make an informed decision.


Ask 3: Why is there a need to convert the LinkedList to a Vec and then back to LinkedList in this
program?
Answer: Here are some general reasons why someone might do this:

    1. Sorting: `LinkedList` in Rust does not have a built-in sort method, but `Vec` does.
    So, if you need to sort a `LinkedList`, you might convert it to a `Vec`, sort the `Vec`, and
    then convert it back.

    2. Random Access: `Vec` allows for efficient random access to elements, while `LinkedList` does
    not. If you need to access elements by their index frequently, it might be more efficient to
    convert the `LinkedList` to a `Vec`.

    3. Deduplication: Similar to sorting, `Vec` has a built-in method for removing consecutive
    duplicate elements, but `LinkedList` does not. If you need to deduplicate a `LinkedList`, you
    might convert it to a `Vec`, deduplicate the `Vec`, and then convert it back.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
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
