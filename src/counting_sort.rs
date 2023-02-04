
pub fn sort(arr: &[u8]) -> Vec<u8> {
    let mut count_arr: [usize; 0x100] = [0; 0x100];
    let mut sorted_vec: Vec<u8> = vec![0; arr.len()];
    for i in arr {
        count_arr[*i as usize] += 1;
    }
    for i in 1..0x100 {
        count_arr[i] += count_arr[i - 1];
    }
    for j in arr.iter().rev() {
        let i = *j as usize;
        sorted_vec[count_arr[i] - 1] = *j; 
        count_arr[i] -= 1;
    }
    sorted_vec
}

