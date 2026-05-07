pub fn print(range: u8){
    let numbers = generate_sequence(range);
    output_sequence(&numbers);
}

pub fn get_dlen(data_struct: Vec<u8>) -> u8 {
    return get_data_length(data_struct);
}

fn output_sequence(numbers: &[u8]){
    for n in numbers.iter(){
        println!("{}", n);
    }
}

fn generate_sequence(range: u8) -> Vec<u8> {
    let mut nums = Vec::new();
    for n in 1..=range {
        nums.push(n);
    }
    return nums;
}

fn get_data_length(data_struct: Vec<u8>) -> u8 {
    let dlen: u8 = data_struct.len().try_into().unwrap();
    return dlen;
}