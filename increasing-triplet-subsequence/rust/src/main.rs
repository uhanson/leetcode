pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut left = i32::MAX;
    let mut middle = i32::MAX;

    for &right in nums.iter() {
        if right < left {
            left = right
        } else if right < middle && right > left {
            middle = right;
        } else if right > middle && right > left {
            return true
        }

        println!("{} {} {}", left, middle, right);
    }

    false
}

fn main() {
    println!("{:?}", increasing_triplet(vec![1,2,3,4,5]));
}
