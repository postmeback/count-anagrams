use std::collections::HashMap;

fn calculate_anagram_count(input_string: &str) -> i32 {
    let mut count = 0;
    let n = input_string.trim().len();

    // HashMap to store the frequency of sorted substrings
    let mut substr_freq: HashMap<String, i32> = HashMap::new();

    // Iterate through all possible substrings
    for i in 0..n {
        for j in i..n {
            // Extract the substring and sort its characters
            let mut chars: Vec<char> = input_string[i..=j].chars().collect();
            chars.sort();
            let sorted_substring: String = chars.into_iter().collect();

            // Update the frequency in the HashMap
            let entry = substr_freq.entry(sorted_substring).or_insert(0);
            *entry += 1;
        }
    }

    // Count the number of anagram pairs
    for &freq in substr_freq.values() {
        count += freq * (freq - 1) / 2;
    }

    count
}

fn main() {
    println!("Enter the string");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("Invalid input");

    let anagram_count: i32 = calculate_anagram_count(&input_string);

    println!("The anagram count is {}", anagram_count);
}
