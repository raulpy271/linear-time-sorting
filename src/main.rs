use linear_sort::bucket_sort;

fn main() {
    let arr: [f64; 7] = [0.6, 0.333, 0.2, 0.8, 0.9, 0.87, 0.65];
    let sorted: Vec<f64> = bucket_sort::sort(&arr);
    for i in &sorted {
        print!("{} ", i);
    }
    println!("");
}