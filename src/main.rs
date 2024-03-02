use rand::{thread_rng, Rng};

fn main() {
    let s = vec![
        String::from("1s"),
        String::from("2s"),
        String::from("3s"),
        String::from("4s"),
        String::from("5s"),
        String::from("6s"),
        String::from("7s"),
        String::from("8s"),
        String::from("9s"),
    ];
    let m = vec![
        String::from("1m"),
        String::from("2m"),
        String::from("3m"),
        String::from("4m"),
        String::from("5m"),
        String::from("6m"),
        String::from("7m"),
        String::from("8m"),
        String::from("9m"),
    ];
    let p = vec![
        String::from("1p"),
        String::from("2p"),
        String::from("3p"),
        String::from("4p"),
        String::from("5p"),
        String::from("6p"),
        String::from("7p"),
        String::from("8p"),
        String::from("9p"),
    ];
    let j = vec![
        String::from("t"),
        String::from("n"),
        String::from("s"),
        String::from("p"),
        String::from("hk"),
        String::from("ht"),
        String::from("tyu"),
    ];
    let mut mountain = Vec::new();
    for _ in 0..4 {
        mountain.extend(s.iter().cloned());
        mountain.extend(m.iter().cloned());
        mountain.extend(p.iter().cloned());
        mountain.extend(j.iter().cloned());
    }

    let mountain_len = mountain.len();
    let mut rng = thread_rng();
    let random_number = rng.gen_range(0..=mountain_len);

    println!("{}", mountain[random_number]);
}
