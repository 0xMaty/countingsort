mod countingsort;

fn main() {
    let unsorted = [100, 1, 2, 5, 25, 33, 15, 99, 0, 75, 200, 3];
    let mut sorted = [0; 12];
    let mut count = [0; 200 + 1];
    println!("Unsorted integers: {:?}", unsorted);
    countingsort::sort(&unsorted, &mut sorted, &mut count);
    println!("Sorted integers: {:?}", sorted);
}
