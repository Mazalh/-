use rand::{thread_rng, Rng};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<((usize, usize), i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_indexes = (0, 1);

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indexes = (i, i + 1);
        }
    }

    Some((min_indexes, min_sum))
}

fn print_min_adjacent_sum(data: &[i32]) {
    println!("indexes : 0.  1.  2.  3.  4.  5.  6.  7.  8.  9. 10. 11. 12. 13. 14. 15. 16. 17. 18. 19.");
    println!("data : {:?}", data);
    println!("indexes : ");

    if let Some(((idx1, idx2), sum)) = min_adjacent_sum(data) {
        println!("min adjacent sum = {} + {} = {} at indexes : {},{}", data[idx1], data[idx2], sum, idx1, idx2);
    } else {
        println!("min adjacent sum = None");
    }
}

fn main() {
    let data = gen_random_vector(20);
    print_min_adjacent_sum(&data);
}
