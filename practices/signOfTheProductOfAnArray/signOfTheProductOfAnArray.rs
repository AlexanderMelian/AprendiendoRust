struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let n_len = nums.len();
        let mut i = 0;
        let mut rta = 1;
        while i < n_len{
            if nums[i]>0{
                rta = rta*1;
                i+=1;
                continue;
            }
            if nums[i]<0{
                rta = rta*-1;
                i+=1;
                continue;
            }
            if nums[i]==0{
                rta = 0;
                break;
            }
            i+=1;
        }
        return rta;
    }
}


fn main(){
    let mut param: Vec<i32> = vec![1,5,0,2,-3];
    let mut res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,3,2,-3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,1,-4,2,-3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![0,5,2,2,-3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,0,2,-3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,1,2,3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,0,0,4];
    res = Solution::array_sign(param);
    println!("{}",res);
    param = vec![1,5,2,-3,4];
    res = Solution::array_sign(param);
    println!("{}",res);
}