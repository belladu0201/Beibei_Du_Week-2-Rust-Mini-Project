use std::collections::HashMap;
// define a function to find the two numbers that add up to the target
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // define a hashmap to store the number and its index
    let mut map = HashMap::new();
    // iterate through the vector
    for (i, num) in nums.iter().enumerate() {
        // find the complement of the number since we only have two numbers
        let complement = target - num;
        // if the complement is in the hashmap, return the index of the complement and the current number
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        // insert the number and its index into the hashmap
        map.insert(num, i);
    }
    // if not found, return an empty vector
    vec![]
}

fn main() {
    // some test cases by LeetCode
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(two_sum(nums, target), vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(two_sum(nums, target), vec![0, 1]);
}

