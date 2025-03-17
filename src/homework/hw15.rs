use std::collections::HashSet;

fn solve_rebus() {
    let mut solutions = 0;

    for m in 1..=9 {
        for u in 0..=9 {
            for h in 0..=9 {
                for a in 1..=9 {
                    for s in 1..=9 {
                        for l in 0..=9 {
                            for o in 0..=9 {
                                for n in 0..=9 {
                                    let mut digits = HashSet::new();
                                    digits.insert(m);
                                    digits.insert(u);
                                    digits.insert(h);
                                    digits.insert(a);
                                    digits.insert(s);
                                    digits.insert(l);
                                    digits.insert(o);
                                    digits.insert(n);

                                    if digits.len() == 8 {
                                        let muha = m * 1000 + u * 100 + h * 10 + a;
                                        let slon = s * 1000 + l * 100 + o * 10 + n;

                                        if muha * a == slon {
                                            println!("Розв'язок знайдено:");
                                            println!("   MUHA");
                                            println!("   {}{}{}{}", m, u, h, a);
                                            println!("* {}", a);
                                            println!("---------");
                                            println!("   SLON");
                                            println!("   {}{}{}{}", s, l, o, n);
                                            println!();
                                            solutions += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if solutions > 0 {
        println!("Кількість розв'язків: {}", solutions);
    } else {
        println!("Розв'язок не знайдено.");
    }
}

fn main() {
    solve_rebus();
}
