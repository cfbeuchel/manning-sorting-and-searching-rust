use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;


// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

#[allow(dead_code, unused_variables)]
impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

// Make a vector of random i32 values in the range [0 and max).
fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// Quicksort
fn quicksort(vec: &mut [i32]){
    // If array has zero or one elements, return immediately
    if vec.len()  < 2 {
        return;
    } else {
        let p = partition(vec);
        quicksort(&mut vec[..p]);
        quicksort(&mut vec[p+1..]);
    }
}

// Takes care of the partitioning step
fn partition(vec: &mut [i32]) -> usize{
    // Index of last element, also called `r`
    let r = vec.len() - 1usize;
    // Index of the pivot point `q`
    // Lomutu partitioning uses the last element
    let pivot = r;
    // Index separating the low from high side (can be negative)
    let mut i = -1;
    // Index traversing array to partition
    for j in 0..pivot {
        // Compare the element with the pivot element
        if vec[j] <= vec[pivot] {
            // Increase separation index
            i += 1i32;
            // Swap values at indices
            vec.swap(i as usize, j);
        }
    }
    let new_pivot: i32 = i + 1;
    vec.swap(new_pivot as usize, pivot);
    return new_pivot as usize;
}

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
    let mut sorted = true;
    for i in 1..vec.len() {
        if let Ordering::Less = vec[i].cmp(&vec[i-1]) {
            sorted = false;
            break;
        }
    }
    match sorted {
        true => println!("The vector is sorted!"),
        false => println!("The vector is NOT sorted!"),
    }
}

fn main() {
    let input_len = get_i32("Enter the desired vector length:\n");
    let input_max = get_i32("Enter the maximum:\n");
    let mut vec_to_sort = make_random_vec(input_len, input_max);
    let print_max = 15;
    println!("Showing the first {print_max} elements of the vector:");
    print_vec(&vec_to_sort, print_max);
    println!("Sorting vector...");
    quicksort(&mut vec_to_sort[..]);
    println!("Showing the first {print_max} elements of the sorted vector:");
    print_vec(&vec_to_sort, print_max);
    check_sorted(&vec_to_sort);
}
