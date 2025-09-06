pub fn mean(list: &[i32]) -> f64 {
    let mut sum = 0;
    for num in list {
        sum += num;
    }
    return sum as f64 / list.len() as f64;
}

pub fn median(list: &[i32]) -> i32 {
    let mut temp = list.to_vec();
    temp.sort();
    if temp.len() % 2 == 0 {
        return (temp[temp.len() / 2] + temp[(temp.len() / 2) - 1]) / 2;
    }
    return temp[temp.len() / 2];
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counter:i32 = 0;
    let mut biggest:i32 = 0;
    for (i, val) in list.iter().enumerate() {
        for (j, val1) in list.iter().enumerate() {
            if i != j && val == val1 {
                counter += 1;
            }
        }
        if counter > biggest {
            biggest = *val;
        }
    }
    return biggest;
}
