fn main() {
    println!("Hello, world!");

    let mut i = 0;
    let mut j = 0;

    while i < 5 {
        while j < 5 {
            let start_time = std::time::Instant::now();
            let result = ackermann(i, j);
            let end_time = std::time::Instant::now();
            let duration = end_time.duration_since(start_time);
            println!("{} {} {} {}ms", i, j, result, duration.as_millis());
            j += 1;
        }

        i += 1;
        j = 0;
    }
}

fn ackermann(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}
