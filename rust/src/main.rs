use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let mut s = vec![];
    let mut m = vec![];
    let mut p = vec![];
    let mut j = vec![];

    for i in 1..=9 {
        s.push(format!(":mahjong-sou{}:", i));
        m.push(format!(":mahjong-man{}:", i));
        p.push(format!(":mahjong-pin{}:", i));
    }

    let wind_tiles = [
        ":mahjong-ton:",
        ":mahjong-nan:",
        ":mahjong-sha:",
        ":mahjong-pei:",
    ];
    let dragon_tiles = [":mahjong-haku:", ":mahjong-hatsu:", ":mahjong-chun:"];
    j.extend(wind_tiles.iter().cloned());
    j.extend(dragon_tiles.iter().cloned());

    let mut mountain = vec![];
    for _ in 0..4 {
        mountain.extend(s.iter().cloned());
        mountain.extend(m.iter().cloned());
        mountain.extend(p.iter().cloned());
        mountain.extend(j.iter().cloned());
    }

    let haipai: Vec<_> = (0..13)
        .map(|_| mountain.choose(&mut rng).unwrap().clone())
        .collect();

    println!("{:?}", haipai);
}
