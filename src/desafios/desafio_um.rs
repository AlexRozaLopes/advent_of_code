
pub fn calibration_values(dados:&str) -> usize {
    dados.lines().filter_map(|linha| {
        let linha = linha
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");
        println!("{}",linha);
        let numeros = || linha.chars().filter(char::is_ascii_digit);
        let primeiro = numeros().next().unwrap();
        let ultimo = numeros().last().unwrap_or_else(|| primeiro);
        
        
        println!("{}{}",primeiro,ultimo);
        
        format!("{primeiro}{ultimo}").parse::<usize>().ok()
        
    }).sum::<usize>()
   
}


#[test]
fn exemplo_desafio_um() {
    let soma = calibration_values("1abc2
pqr3stu8vwx
a1b2c3d4e5f
sevenine
treb7uchet");
    assert_eq!(221,soma)
}
