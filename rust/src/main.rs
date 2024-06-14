use rand::Rng;

fn main() {
    let nome: &str = "Mansa Musa";
    let mut rng = rand::thread_rng();
    let xp: i32 = rng.gen_range(0..10001);
    let nvl = ["Ferro", "Bronze", "Prata", "Ouro", "Platina", "Ascendente", "Imortal", "Radiante"];

    let _nvl = if xp <= 1000 {
        nvl[0]
    } else if xp <= 2000 {
        nvl[1]
    } else if xp <= 5000 {
        nvl[2]
    } else if xp <= 7000 {
        nvl[3]
    } else if xp <= 8000 {
        nvl[4]
    } else if xp <= 9000 {
        nvl[5]
    } else if xp <= 10000 {
        nvl[6]
    } else {
        nvl[7]
    };

    println!("O herói {} está no nível {}", nome, _nvl);
}
