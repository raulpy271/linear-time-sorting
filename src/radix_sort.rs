
fn get_byte(number: u64, byte_index: usize) -> u8 {
    if byte_index >= 8 {
        panic!("A u64 number only have 8 bytes, so byte_index should be 0..=7");
    } else {
        let bits_to_shift = 8 * byte_index;
        let byte_with_pads_on_right: u64 = number & (0xff << bits_to_shift);
        (byte_with_pads_on_right >> bits_to_shift).try_into().unwrap()
    }
}

pub fn sort(arr: &[u64]) -> Vec<u64> {
    // For each 8-byte number it sorts the first byte, and then the second and so on until sort all 8 bytes.
    let mut count_arr: [usize; 0x100] = [0; 0x100];
    let mut temp_vec: Vec<u64> = arr.to_vec();
    let mut sorted_vec: Vec<u64> = vec![0; arr.len()];
    let mut arr_ref: &mut [u64] = &mut temp_vec[..];
    let mut sorted_ref: &mut [u64] = &mut sorted_vec[..];
    for byte_n in 0..8 {
        // The following is the count sort algorithm adapted to sort by a single byte instead of the whole number
        for i in 1..0x100 {
            count_arr[i] = 0;
        }
        for i in (&arr_ref).iter() {
            let byte = get_byte(*i, byte_n) as usize;
            count_arr[byte] += 1;
        }
        for i in 1..0x100 {
            count_arr[i] += count_arr[i - 1];
        }
        for j in (&arr_ref).iter().rev() {
            let byte = get_byte(*j, byte_n) as usize;
            sorted_ref[count_arr[byte] - 1] = *j; 
            count_arr[byte] -= 1;
        }
        let arr_ref_temp = arr_ref;
        arr_ref = sorted_ref;
        sorted_ref = arr_ref_temp;
    }
    // In the last iteration of the loop, sorted_ref was pointed to temp_vec, so temp_vec is the sorted vector
    temp_vec
}