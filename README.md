
# Sorting algorithms that run in linear time

This repository implements in Rust some algorithms that are known to run in linear time. For a basic understanding of these algorithms, see the [References section](#References).

The algorithms are:

 - **Count sort**: it sorts a array of unsigned 1-byte number, which is `u8`, in rust.
 - **Radix sort**: it sorts a array of unsigned 8-byte number, which is `u64`, in rust.

# Usage example

### Count sort


```rs
use linear_sort::counting_sort;

fn main() {
    let arr: [u8; 7] = [8, 2, 3, 100, 38, 3, 8];
    let sorted: Vec<u8> = counting_sort::sort(&arr);
    for i in &sorted {
        print!("{} ", i);
    }
    println!("");
}
```

```sh
$ cargo run
2 3 3 8 8 38 100 
```

### Radix sort

```rs
use linear_sort::radix_sort;

fn main() {
    let arr: [u64; 7] = [8000, 3000, 38, 100, 38, 3, 3000];
    let sorted: Vec<u64> = radix_sort::sort(&arr);
    for i in sorted {
        print!("{} ", i);
    }
    println!("");
}
```

```sh
$ cargo run
3 38 38 100 3000 3000 8000 
```

# References

- Introduction to algorithms / Thomas H. Cormen / Section 8
