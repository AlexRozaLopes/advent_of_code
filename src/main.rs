pub mod desafios;

fn main() {
    let resultado = desafios::desafio_um::calibration_values(include_str!("./dados/desafio_um.txt"));
    println!("resposta do desafio um -> {}", resultado);

    let resultado_dois = desafios::desafio_dois::valid_game(include_str!("./dados/desafio_dois.txt"));
    println!("resposta do desafio um -> {}", resultado_dois);
}
