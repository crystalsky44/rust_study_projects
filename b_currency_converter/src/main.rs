use std::io;
use std::io::Write;

fn main() {
    print!("input a number! ");

    let usd_eur_rate: f32 = 0.86;

    let mut input = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim_end().parse::<f32>().unwrap() * usd_eur_rate;

    print!("your input: {input}");

}

fn truncate_at_hundreth(floating_point: &mut f32) -> f32 {
    let truncated: f32 = (*floating_point * 100.0).trunc() / 100.0;
    truncated
}
