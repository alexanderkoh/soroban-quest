// No incorporamos la biblioteca standard para minimizar el tamaño de compilación.
// Importamos unos cuantos macros y tipos (types) desde la `soroban_sdk`.
#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct HelloContract;

// Nuestra implementación del contrato `HelloContract` contiene solo una función, `hello()`.
// Esta función recive un argumento `to` y retorna un Vec made compuesto de
// "Hello" y el valor suministrado al argumento `to`.
#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

// Esta declaración `mod` inserta los contenidos de `test.rs` en este archivo.
mod test;