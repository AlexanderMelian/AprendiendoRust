//https://leetcode.com/problems/reverse-string/

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let s_len = s.len();
        let mut help;
            let mut i = 0;
            while i < (s_len/2){
                help = s[i];
                s[i] = s[s_len-1-i];
                s[s_len-1-i] = help;
                i += 1;
        }
    }
}


fn main(){
    let mut param: Vec<char> = vec!['a','e'];
    Solution::reverse_string(&mut param);
    println!("{:?}",param);
}