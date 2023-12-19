
pub fn valid_game(dados: &str) -> usize {
    let mut soma : usize = 0;
    let mut jogos_validos : Vec<usize> = Vec::new();
    let mut jogos_invalidos : Vec<usize> = Vec::new();
    dados.lines().for_each(|linha| {
        let (game_id, game) = linha.split_once(":").unwrap();
        let (_, id) = game_id.split_once(" ").unwrap();
        game.lines().for_each(|game| {
            game.split(";").for_each(|rgb| {
                rgb.split(",").for_each(|cor| {
                    let (numero, nome) = cor.rsplit_once(" ").unwrap();
                    match nome.contains("red") && numero.trim().parse::<usize>().unwrap() < 12
                        || nome.contains("green") && numero.trim().parse::<usize>().unwrap() < 13
                        || nome.contains("blue") && numero.trim().parse::<usize>().unwrap() < 14
                    {
                        true => jogos_validos.push(id.parse::<usize>().unwrap()),
                        false => jogos_invalidos.push(id.parse::<usize>().unwrap()),
                    };
                })
            });
            jogos_validos.dedup();
            jogos_invalidos.dedup();
            
            jogos_invalidos.iter().for_each(|numero| {
                jogos_validos.retain(|n| n.clone() != numero.clone());
            });
            soma = jogos_validos.iter().sum();
        });
    });
    soma
}


#[test]
fn exemplo_desafio_dois() {
    let texto = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(8, valid_game(texto))
}
