pub mod desafios;

fn main() {
    let resultado = desafios::desafio_um::calibration_values(include_str!("./dados/desafio_um.txt"));
    println!("resposta do desafio um -> {}", resultado);
}
