pub fn draw_christmas_tree(num_triangles: usize) {
    let max_width = num_triangles * 2 - 1;

    for i in 1..=num_triangles { // Змінено на прямий порядок
        let width = i * 2 - 1;
        let spaces = (max_width - width) / 2;

        for _ in 0..i {
            print!("{:spaces$}", "");
            for _ in 0..width {
                print!("*");
            }
            println!();
        }
    }
}
