use std::{io, io::prelude::*};

pub fn event() -> i32 {
    print!("Digite um opção e tecle <ENTER>: ");
    io::stdout().flush().unwrap();
    let mut input_data: String = String::new();
    io::stdin()
        .read_line(&mut input_data)
        .expect("Erro ao ler o evento reader");
    match input_data.trim().parse::<i32>() {
        Ok(v) => v,
        Err(_) => -1,
    }
}
