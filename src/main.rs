mod build_array_from_permutation;
mod concatenation_arrays;
mod count_items_matching_rule;
mod create_target_array;
mod defang_ip_addr;
mod final_value_after_operations;
mod jewels_and_stones;
mod convert_binary_number_linked_list;
mod decode_message;
mod decode_xored_array;
mod decompress_run_len_encoded_list;
mod kids_with_candies;
mod max_number_of_words_in_sentences;
mod min_moves_to_seat;
mod minimum_sum_of_four_digits;
mod number_of_good_pairs;
mod number_of_matches;
mod number_of_steps;
mod richest_customer_wealth;
mod running_sum_of_array;
mod shuffle_string;
mod shuffle_the_array;
mod smaller_numbers_than_current;
mod sorting_sentence;
mod substract_product_and_sum;

fn main() {
    println!("Hello, world!");
    // Rust strings use the least amount of bytes possible to represent its chars
    println!("Size of char = {}", std::mem::size_of::<char>());
    println!("Size of \"a\" = {}", "a".len());
    println!("Size of \"ß\" = {}", "ß".len());
    println!("Size of \"国\" = {}", "国".len());
    println!("Size of \"𓅱\" = {}", "𓅱".len());
    println!("Size of \"안녕\" = {}", "안녕".len());
}
