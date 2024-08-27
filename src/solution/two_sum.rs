use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    for (i, val) in nums.iter().enumerate() {
        map.insert(val, i as i32);
    }

    for i in 0..nums.len() as i32 {
        let likely_answer = target - nums[i as usize];

        if map.contains_key(&likely_answer) && (map[&likely_answer] != i) {
            return vec![i, map[&likely_answer]];
        }
    }

    return Vec::with_capacity(0);
}
