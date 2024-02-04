/*
Ask 1: How does the calculate_weights function determine the weight of each language?
Answer: the calculate_weights function determines the weight of each language according to its
age (year of creation) relative to now.
The older the language, the greater its weight.


Ask 2: How does the code ensure that the weights are normalized between 1 and 100?
Answer: The code will query the HashMap to find the min and max values, and in the event of failure
set them to 0.

The code is performing a normalization operation on a variable year. Normalization is a technique often used in data
processing to change the values of numeric columns in a dataset to a common scale, without distorting differences in
the ranges of values or losing information.

Here's a of what the code does:

    1. (year - min_year) as f64: This operation subtracts the minimum year (min_year) from the given year (year).
    The result is the distance of year from the minimum year.

    2. (max_year - min_year) as f64: This operation subtracts the minimum year from the maximum year.
    This gives the total range of years.

    3. (year - min_year) as f64 / (max_year - min_year) as f64: Finally, the code divides the distance of year from the
    minimum year by the total range of years.
    This results in a normalized value for year that falls between 0 and 1 (inclusive).


Ask 3: Why does the code use values_mut when updating the years in the calculate_weights function?
Answer: The values_mut function is used in Rust when you want to get a mutable reference to the values of a collection,
such as a Vector or a HashMap.

In Rust, variables are immutable by default. This means that once a variable is initialized with a value, you can't
change that value. However, you can override this behavior by using the mut keyword, which allows you to change the
value of a variable.
*/

use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Subtract the creation year from 2024 to get the number of years active.
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
