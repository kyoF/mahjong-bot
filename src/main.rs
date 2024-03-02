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
        String::from("to"),
        String::from("na"),
        String::from("sy"),
        String::from("pe"),
        String::from("hk"),
        String::from("ht"),
        String::from("ty"),
    ];
    let mut mountain = Vec::new();
    for _ in 0..4 {
        mountain.extend(s.iter().cloned());
        mountain.extend(m.iter().cloned());
        mountain.extend(p.iter().cloned());
        mountain.extend(j.iter().cloned());
    }

    let mut haipai = Vec::new();
    for _ in 0..13 {
        let mountain_len = mountain.len();
        let mut rng = thread_rng();
        let rand_idx = rng.gen_range(0..=mountain_len-1);
        let random_pai = mountain.remove(rand_idx);
        haipai.push(random_pai);
    }
    haipai.sort_by(|a, b| {
        let a_starts_with_digit = a.chars().next().unwrap().is_numeric();
        let b_starts_with_digit = b.chars().next().unwrap().is_numeric();

        // 一文字目が数字のものを優先する
        if a_starts_with_digit && !b_starts_with_digit {
            std::cmp::Ordering::Less
        } else if !a_starts_with_digit && b_starts_with_digit {
            std::cmp::Ordering::Greater
        } else {
            // 数字がついている文字列は二文字をソートする
            if a_starts_with_digit && b_starts_with_digit {
                // 二文字目を優先して比較し、同じ場合は一文字目で比較する
                let a_second_char = a.chars().nth(1).unwrap();
                let b_second_char = b.chars().nth(1).unwrap();
                let cmp_second_char = a_second_char.cmp(&b_second_char);
                if cmp_second_char != std::cmp::Ordering::Equal {
                    cmp_second_char
                } else {
                    a.chars().next().unwrap().cmp(&b.chars().next().unwrap())
                }
            } else {
                // どちらも数字またはどちらも数字でない場合は通常の比較を行う
                a.cmp(b)
            }
        }
    });

    println!("{:?}", haipai);
}
