use std::collections::HashMap;

pub fn median(list: Vec<u32>) -> u32 {
    let len = list.len();
    let half = len / 2;

    if half % 2 != 0 {
        let v = list.get(half);
        return match v {
            Some(v) => *v,
            None => panic!("error"),
        }
    }

    let before = list.get(half - 1).unwrap();
    let after = list.get(half).unwrap();

    (before + after) / 2
}

pub fn mode(list: Vec<u32>) -> u32 {
    let mut map = HashMap::new();

    for item in list {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut max_key = 0;
    for (key, value) in map {
        if value > max {
            max = value;
            max_key = key;
        }
    }

    max_key
}

#[cfg(test)]
mod tests {
    use crate::median::*;

    #[test]
    fn test_median() {
        let result = median(vec![1, 2, 3]);
        assert_eq!(result, 2);

        let result = median(vec![10, 20, 30, 40]);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_mode() {
        let result = mode(vec![10, 10, 20, 10]);
        assert_eq!(result, 10);

        let result = mode(vec![1, 2, 3, 4, 5, 4]);
        assert_eq!(result, 4);
    }
}
