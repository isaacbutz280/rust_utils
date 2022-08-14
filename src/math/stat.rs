pub fn mean(vec: &Vec<i32>) -> Option<f64> {
    let mut sum = 0.0;
    if vec.len() == 0 {
        None
    } else {
        for &n in vec {
            sum += n as f64;
        }
        Some(sum / (vec.len() as f64))
    }
}

pub fn median(vec: &mut Vec<i32>) -> Option<f64> {
    vec.sort();
    let length = vec.len();
    if length == 0 {
        None
    } else {
        let med = if length % 2 == 0 {
            (vec.get(length / 2), vec.get(length / 2 - 1))
        } else {
            (vec.get(length / 2), vec.get(length / 2))
        };
        match med {
            (Some(i), Some(j)) => Some((i + j) as f64 / 2.0),
            (_, _) => None,
        }
    }
}

pub fn mode(vec: &Vec<i32>) -> Option<i32> {
    let mut hm = std::collections::HashMap::new();
    let mut m = 0;
    for n in vec {
        // Or insert returns a mutable ref
        let count = hm.entry(n).or_insert(0);
        *count += 1;
        if *count > m {
            m = *n;
        }
    }    if vec.len() == 0 {
        None
    } else {
        Some(m)
    }
}
