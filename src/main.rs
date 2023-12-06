use std::cmp::Ordering;

use clap::Parser;
use reqwest::Error;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct CepResponse {
    cep: String,
    logradouro: String,
    complemento: String,
    bairro: String,
    localidade: String,
    uf: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String,
}

async fn get_cep_data(cep: &str) -> Result<CepResponse, Error> {
    let url = format!("https://viacep.com.br/ws/{}/json/", cep);
    let response = reqwest::get(&url).await?.json::<CepResponse>().await?;
    Ok(response)
}
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short)]
    cep: String,
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let cep = &Cli::parse().cep; // Exemplo de CEP, você pode substituir pelo CEP desejado
    match cep.capacity().cmp(&8) {
         Ordering::Equal => {
            print!("Searching about cep {}\n", cep)
        }
        _ => {
            panic!("Hey bro, need to be like this pattern 00100100")
        }
       
    }
    match get_cep_data(cep).await {
        Ok(data) => {
            println!("CEP: {}", data.cep);
            println!("Logradouro: {}", data.logradouro);
            println!("Complemento: {}", data.complemento);
            println!("Bairro: {}", data.bairro);
            println!("Cidade: {}", data.localidade);
            println!("Estado: {}", data.uf);
            println!("IBGE: {}", data.ibge);
            println!("GIA: {}", data.gia);
            println!("DDD: {}", data.ddd);
            println!("SIAFI: {}", data.siafi);
        }
        Err(e) => eprintln!("Erro: {:?}", e),
    }

    Ok(())
}
