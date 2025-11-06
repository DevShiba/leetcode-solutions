impl Solution {
    fn get_sum(counter: &[i32; 51], x: i32) -> i32 {
        let mut heap = counter.iter()
            .enumerate()
            .filter(|(i, count)| **count != 0)
            .map(|(i, count)| (count, i as i32))
            .collect::<std::collections::BinaryHeap<_>>();

        let mut result = 0;
        for _ in 0..x {
            if let Some((count, num)) = heap.pop() {
                result += count * num;
            } else {
                break;
            }
        }

        result        
    }

    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut counter = [0; 51];
        let mut result = vec![0; nums.len() - k + 1];
        for i in 0..nums.len() {
            counter[nums[i] as usize] += 1;
            if i < k - 1 {
                continue;
            }

            result[i - k + 1] = Self::get_sum(&counter, x);
            counter[nums[i - k + 1] as usize] -= 1;
        }

        result
    }
}