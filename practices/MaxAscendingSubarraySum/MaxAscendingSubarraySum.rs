struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let n_len = nums.len();
        if n_len == 1{
            return nums[0];
        }
        let mut i = 1;
        let mut tmp = nums[0];
        let mut max = 0;
        while i < n_len{
            if nums[i] > nums[i-1]{
                tmp += nums[i];
            }
            else{
                if tmp > max{
                    max = tmp;
                }
                tmp = nums[i]
            }
            i+=1;
        }
        if tmp > max{
            return tmp;
        }
        return max;
    }
}


fn main(){
    let mut x = vec![10,20,30,5,10,50];
    println!("{}",Solution::max_ascending_sum(x));
    x = vec![10,20,30,40,50];
    println!("{}",Solution::max_ascending_sum(x));
    x = vec![12,17,15,13,10,11,12];
    println!("{}",Solution::max_ascending_sum(x));
    x = vec![100,10,1];
    println!("{}",Solution::max_ascending_sum(x));
}