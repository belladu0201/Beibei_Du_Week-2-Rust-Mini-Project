# Week4_Rust-Mini_project
## This Rust mini-project targets to use Rust language to answer leetcode questions.

## Problem Description
The "Two Sum" problem is a classic coding challenge in Leetcode (Question1). The goal of the problem is to find two numbers in an array that add up to a given target value, and return their indices in the array. If there isn't a list of index fullfill that requirements, then return a blank list/vector/array.

To solve the problem, we need write a function that takes in an array of integers and a target value as input. The function should then iterate through the array and check whether any two numbers in the array add up to the target value. Once you have identified the two numbers, you should return their indices in the array as a vector.

## Sample output
Input array is [2, 7, 11, 15] and the target value is 9, the function should return [0, 1] because the numbers 2 and 7 add up to 9, and their indices in the array are 0 and 1, respectively.

## Defined function
```
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
```

## Reference
- ChatGPT
- Copilot
- https://leetcode.com/problems/two-sum/
