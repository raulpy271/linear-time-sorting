
use linear_sort;

fn main() {
    let arr: [u8; 11] = [34, 23, 10, 200, 0, 250, 255, 2, 10, 2, 255];
    let sorted: Vec<u8> = linear_sort::counting_sort::sort(&arr);
    println!("Hello, world!");
    for i in sorted {
        print!("{} ", i);
    }
}
