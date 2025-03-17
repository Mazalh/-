use rand::{Rng, thread_rng};

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total_weight: u32 = shipments.iter().sum();
    let num_ships = shipments.len() as u32;

    if total_weight % num_ships != 0 {
        return -1; // Повертаємо -1, якщо розподіл неможливий
    }

    let average_weight = total_weight / num_ships;
    let mut moves = 0;
    let mut diffs: Vec<i32> = shipments
        .iter()
        .map(|&s| s as i32 - average_weight as i32)
        .collect();

    for i in 0..diffs.len() - 1 {
        if diffs[i] != 0 {
            diffs[i + 1] += diffs[i];
            moves += diffs[i].abs();
        }
    }

    moves
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let average_weight = rng.gen_range(1..10) * 10; // Випадкова середня вага
    let mut shipments = Vec::with_capacity(n);

    for _ in 0..n {
        shipments.push(average_weight); // Всі кораблі мають однакову вагу
    }

    shipments
}

fn main() {
    let n = 5; // Кількість кораблів
    let shipments = gen_shipments(n);

    println!("Ваги вантажу на кораблях: {:?}", shipments);

    let moves = count_permutation(&shipments);
    if moves == -1 {
        println!("Неможливо розподілити вантаж однаково.");
    } else {
        println!("Мінімальна кількість переміщень: {}", moves);
    }
}

#[test]
fn test_count_permutation() {
    let shipments = vec![1, 1, 1, 1, 6];
    assert_eq!(count_permutation(&shipments), 4);

    let shipments = vec![1, 2, 3];
    assert_eq!(count_permutation(&shipments), 2);

    let shipments = vec![1, 1, 1, 2];
    assert_eq!(count_permutation(&shipments), -1);
}
