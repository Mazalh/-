fn main() {
    const H: u32 = 11;
    const W: u32 = 11;
    let x_cent = W / 2;
    let y_cent = H / 2;

    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            let symbol = get_symbol(x, y, x_cent, y_cent);
            output.push_str(symbol);
        }
        output.push('\n');
    }

    println!("{}", output);
}

fn get_symbol(x: u32, y: u32, x_cent: u32, y_cent: u32) -> &'static str {
    let dist_x = (x as i32 - x_cent as i32).abs();
    let dist_y = (y as i32 - y_cent as i32).abs();

    match dist_x + dist_y <= y_cent as i32 {
        true => "*",
        false => " ",
    }
}
