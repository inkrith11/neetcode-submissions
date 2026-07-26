impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in &nums{
            let count = map.entry(i).or_insert(0);
                *count+=1;
                if *count == 2 { return true;}
        }    
        return false;
    }
}

