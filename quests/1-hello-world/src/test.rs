#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, vec, Env};

// El propósito de este archivo es ejecutar pruebas automatizadas en el código de contrato 
// que hemos escrito en lib.rs. Escribir pruebas puede ser un tema bastante grande y 
// profundizaremos más en una misión a futuro. ¡Atento!
#[test]
fn test() {
    // registramos el contrato en un entorno (environment) Soroban y construimos 
    // un cliente que podamos usar para invocar el contrato
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // A continuación, llamamos a `client.hello()`, proporcionando "Dev" como nuestro argumento `to`.
    let words = client.hello(&symbol!("Dev"));
    
    // Afirmamos que el contrato debe devolver un Vec que coincida 
    // con lo que esperaríamos recibir de nuestro contrato: [Symbol("Hello"), Symbol("Dev")]
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
