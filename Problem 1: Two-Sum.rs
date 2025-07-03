use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen_numbers: HashMap<i32, usize> = HashMap::new();
        let mut result: Vec<i32> = vec![0];

        for (index, &value) in nums.iter().enumerate(){
            // calculates compilment for searching purposes
            let compliment = target - value;
            if let Some(&compliment_index) = seen_numbers.get(&compliment) {
                // add value into result vector for function
                println!("index: {} value: {}", &compliment_index, &compliment);
                println!("index: {} value: {}", &index, value);
                result = vec![compliment_index as i32, index as i32];
                return result;
            }
            seen_numbers.insert(value , index);
        }
        return result;
    }

fn main() {

    let x: Vec<i32> = vec![1,2,3,4,5,6];
    let target: i32 = 9;
    let y: Vec<i32> = two_sum(x, target);
    println!("{:?}", y);
}
