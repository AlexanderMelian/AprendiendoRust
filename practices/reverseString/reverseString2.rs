//https://leetcode.com/problems/reverse-string/
//this is using recursion
struct Solution;

impl Solution {
    pub fn helper(s: &mut Vec<char>,mut left: usize,mut right: usize){
        if left>=right {
            return;
        }
        let tmp : char = s[left];
        s[left] = s[right];
        left += 1;
        s[right] = tmp;
        right -= 1;
        Solution::helper(s, left, right);
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        let s_len = s.len();
        Solution::helper(s, 0, s_len-1);
    }
}


fn main(){
    let mut param: Vec<char> = vec!['a','e','i','o','u'];
    Solution::reverse_string(&mut param);
    println!("{:?}",param);
}