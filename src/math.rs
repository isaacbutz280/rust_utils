pub mod stat;

pub fn fibonacci(index: u32) -> u128 {
    let mut counter = index;
    let mut sum1 = 0;
    let mut sum2 = 1;
    if counter == 0 {
        0
    } else if counter == 1 {
        1
    } else {
        counter -= 1;
        while counter > 0 {
            let temp = sum2;
            sum2 = sum2 + sum1;
            sum1 = temp;
            counter -= 1;
        }
        sum2
    }
}