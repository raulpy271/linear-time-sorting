
pub fn sort(arr: &[f64]) -> Vec<f64> {
    let mut buckets: Vec<Vec<f64>> = Vec::new();
    for f in arr {
        if (*f) >= 0.0 && (*f) < 1.0 {
            buckets.push(Vec::new());
        } else {
            panic!("The input array should have elements in range [0..1)")
        }
    }
    for f in arr {
        let i = ((*f) * (arr.len() as f64) ).floor() as usize;
        buckets[i].push(*f);
    }
    for i in 0..arr.len() {
        // To sort each bucket I'm using the default unstable algorithm, which is "Pattern-defeating quicksort"
        // As each bucket will have a small size, the algorithm can be insertion sort.
        buckets[i].sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    }
    buckets.concat()
}