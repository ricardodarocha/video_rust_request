use error_chain::error_chain;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
} 

use log::error;

fn main() -> Result<()> {
    env_logger::init();
    
    let cep = match std::env::args().nth(1) {
        Some(valor) => valor,
        None => {
            error!("Parâmetro não foi informado cargo run <CEP>\n");
            panic!();
        }
    };
    
    let url = format!(
        "https://viacep.com.br/ws/{}/json/", cep);

    let mut res = reqwest
        ::blocking
        ::get(url)?;
        
    println!("Status {}", res.status() );
    println!("Headers \n {:#?}", res.headers());

    res.copy_to(&mut std::io::stdout());

    Ok(())
}