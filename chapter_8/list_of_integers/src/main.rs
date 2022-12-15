use rand::Rng;
use sort::quicksort;
use std::collections::HashMap;

fn main() {
    let mut list_of_integers: Vec<u32> = Vec::new();

    for i in 1..101 {
        list_of_integers.push(rand::thread_rng().gen_range(51..=100));
    }

    quicksort(&mut list_of_integers);

    println!("The median is {}", get_median(&list_of_integers));

    let mode = get_mode(&list_of_integers);
    println!("The mode is {:?} and it etc {:?}", mode[0], mode[1]);

}

fn get_median(integers: &Vec<u32>) -> u32  {
    integers[integers.len() / 2]
}

fn get_mode(integers: &Vec<u32>) -> [u32; 2] {

    let mut number_of_coincidences: HashMap<u32, u32> = HashMap::new();
    let mut max_key = 0;
    let mut max_key_val = 0;

    for i in integers {
        *number_of_coincidences.entry(i.clone()).or_insert(0) += 1;
        let v = number_of_coincidences.entry(*i).or_default();
        if v > &mut max_key_val { max_key = *i; max_key_val = *v }
    }

    [max_key, max_key_val]
}