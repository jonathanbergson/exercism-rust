pub fn nth(n: u32) -> u32 {
    let mut cache: Vec<u32> = vec![];

    (2..)
        .filter(|candidate| {
            if !cache.iter().any(|i| candidate % i == 0) {
                cache.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
