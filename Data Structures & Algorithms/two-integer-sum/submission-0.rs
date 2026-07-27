impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len(){
            let diff = target-nums[i];

            if map.contains_key(&diff){
                return vec![map[&diff], i as i32];
            }

            map.insert(nums[i], i as i32);

        }

        vec![]
    }
}
