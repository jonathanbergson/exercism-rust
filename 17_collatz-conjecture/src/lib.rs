pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut rest = n;
    let mut steps = 0;

    while rest > 1 {
        if rest.is_multiple_of(2) {
            rest /= 2;
        } else {
            rest *= 3;
            rest += 1;
        }
        steps += 1;
    }

    Some(steps)
}
