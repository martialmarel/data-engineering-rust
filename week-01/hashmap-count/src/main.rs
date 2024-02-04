/*
This example code counts the frequency of each number in the vector.
 */

/*
Ask 1: How is HashMap used in this program and what is its function?
Answer: For each entry in the numbers array passed as an argument, if the number doesn't exist in
the HashMap, the key is created with a frequency initialized to 1.
If the entry already exists, the frequency is incremented.


Ask 2: Why is or_insert(0) used in frequencies.entry(num).or_insert(0)?
Answer: In Rust, `HashMap`'s `entry(key)` method is used to get a mutable reference to the value for
this key. If the key does not exist, it will return an `Entry` enum that represents a vacant entry.

The `or_insert(default_value)` method is then used on this `Entry`. It returns a mutable reference
to the value of the key if it exists. If the key does not exist, it inserts the `default_value`
(in this case, 0) for the key and returns a mutable reference to the new value.

So, `frequencies.entry(num).or_insert(0)` is used to get the value for the key `num` in the
`HashMap` `frequencies`. If `num` does not exist in the `HashMap`, it inserts `0` as its value.
This is particularly useful when you want to initialize a counter or accumulator in a `HashMap`.


Ask 3: How does the program ensure that each number and its frequency are correctly paired in the
final result?
Answer: By iterating over the entries in the HashMap is to collect the entries and values as tuples
and add them to a simple array
*/

use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
