use std::collections::HashMap;

pub fn median(v: &Vec<i32>) -> Option<&i32> {
    let median_index = (v.len() as f64 / 2.0).ceil() as usize;
    v.get(median_index - 1)
}

pub fn mode(v: &Vec<i32>) -> Option<&i32> {
    let mut count_map: HashMap<&i32, u32> = HashMap::new();

    for element in v {
        let count = count_map.entry(element).or_insert(0);
        *count += 1;
    }

    let mut max_value: u32 = 0;
    let mut max_key: Option<&i32> = None;
    for (key, value) in count_map {
        if value >= max_value {
            max_value = value;
            max_key = Some(key);
        }
    }

    max_key
}

