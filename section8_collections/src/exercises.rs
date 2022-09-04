use std::collections::HashMap;

pub fn exercises() {
    let mut ints = vec![1, 9, 10, 11, 12, 12, 13, 1, 8, 7, 5, 5, 0, 8, 3, 3, 5, 5];
    ints.sort();
    println!("{:?}", ints);
    let median = &ints[ints.len() / 2];
    println!("The median of the list is {}", median);

    let mut mode: Vec<(&i32, i32)> = ints
        .iter()
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry(i).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    mode.sort_by(|a, b| b.1.cmp(&a.1));
    let mode = mode[0].0;

    println!("The mode of the list is {}", mode);
    let another_mode = {
        let mut max = 0;
        let mut max_count = 0;
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for k in ints {
            let i = counts.entry(k).or_insert(0);
            *i += 1;
            if *i > max_count {
                max = k;
                max_count = *i;
            }
        }
        max
    };
    println!("Another way to get the mode: {}", another_mode);
}
