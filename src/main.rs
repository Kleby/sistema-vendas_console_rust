mod events;
mod models;
mod renderers;

use events::reader;
use models::{product::ProductItem, sale::Sale, user::User};
use renderers::{console, menu};

fn main() {
    let mut select: String;
    loop {
        menu::show();
        select = (match reader::event() {
            1 => "Escolheu 1",
            2 => "Escolheu 2",
            3 => "Escolheu 3",
            4 => "Escolheu 4",
            5 => "Escolheu 5",
            6 => "Escolheu 6",
            0 => {
                "Escolheu 0";
                break;
            }
            _ => "\nOpção inválida!",
        })
        .to_string();
        console::clear();
        println!("{}", select);
    }
}
