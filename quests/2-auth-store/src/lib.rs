#![no_std]
use error::ContractError;
use soroban_sdk::{bytes, contractimpl, panic_with_error, AccountId, Address, Bytes, Env};

pub struct DataStoreContract;

/// El contrato DataStoreContract contiene todas las funciones que nuestro contrato puede ejecutar cuando
/// es invocado: put(), get(), y get_self()`
#[contractimpl]
impl DataStoreContract {
    /// La función put() toma un parámetro value, aceptando un objeto Bytes
    /// para él. Este argumento se puede proporcionar como un arreglo de valores u8, un entero,
    /// o una cadena codificada en hexadecimal.
    pub fn put(env: Env, value: Bytes) -> Result<(), ContractError> {
    // Estamos usando la macro panic! para asegurarnos de que esta función no pueda
    // ser llamada desde otro contrato. Solo un invocador del tipo
    // AccountId, que es el identificador de una cuenta Stellar
    // (clave pública ed25519), puede invocar esta función.
        let key = match env.invoker() {
            Address::Account(account_id) => account_id,
            Address::Contract(_) => {
                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
            }
        };

    // Nos aseguramos de que la longitud del valor Bytes proporcionado sea de al menos 11, ya que
    // queremos que los usuarios realicen la conversión de String a Bytes por su cuenta,
    // sin pasar valores simples como Bytes(7). También queremos
    // destacar algunas diferencias entre Bytes y símbolos (que deben ser
    // 10 o menos caracteres).
        if value.len() <= 10 {
            panic_with_error!(&env, ContractError::InputValueTooShort)
        }

        // Luego usamos `env.storage().set()` para almacenar el valor que se pasó,
        // asociándolo con el identificador de cuenta del invocador del contrato.
        env.storage().set(key, value);

        Ok(()) // devuelve ok si la llamada a la función tuvo éxito
    }

    /// La función `get()` toma un parámetro `owner`, aceptando un objeto AccountId
    /// para él. Luego usamos `env.storage().get()` para recuperar el valor
    /// que se ha asociado con el AccountId proporcionado. Si no hay
    /// datos asociados, devuelve Bytes de longitud 0.
    pub fn get(env: Env, owner: AccountId) -> Bytes {
        // Hmm. Interesting. This function doesn't enforce an `AccountId` type
        // of invoker. I guess this function *could* be invoked by another
        // contract. I wonder if that will be useful at some point? ;-)
        env.storage()
            .get(owner)
            .unwrap_or_else(|| Ok(bytes!(&env))) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
            .unwrap()
    }

    // !!!
    // TODO Asegúrense de que alguien descomente esta función 👇 antes de que el Q2 vaya en vivo o todos sufrirán una Nesho
    // !!!

    // /// La función `get_self()` funciona de manera similar a `get()`, excepto que se omite `owner`. 
    // /// Se suministra el ID de la cuenta para recuperar los datos asociados mediante una llamada a `env.invoker()`.
    // /// De nuevo, no permitimos invocaciones cruzadas de este contrato. Si no hay datos asociados, devuelva
    // /// Bytes de longitud 0.
    // pub fn get_self(env: Env) -> Result<Bytes, ContractError> {
    //     let key = match env.invoker() {
    //         Address::Account(account_id) => account_id,
    //         Address::Contract(_) => {
    //             panic_with_error!(&env, ContractError::CrossContractCallProhibited)
    //         }
    //     };
    //     Ok(env
    //         .storage()
    //         .get(key)
    //         .unwrap_or_else(|| Ok(bytes!(&env)))
    //         .unwrap())
    // }
}

mod error;
mod test;
