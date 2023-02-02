#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Accounts, BytesN, Env};

/// Estos tests son mucho más interesantes y mucho más involucrados que la
/// primera tarea, así que profundicemos en ellos un poco más. Tenemos dos
/// funciones de prueba que están probando como si el contrato fuera llamado por una AccountId
/// o por un Contrato.

/// La primera función, test_store(), probará los valores que se están
/// almacenando por nuestro contrato. Esto se logra generando un par de cuentas de usuario,
/// almacenando datos como esos usuarios y asegurándose de que los datos recuperados coincidan
/// con lo que esperaríamos que fuera. También estamos verificando contra una pareja de claves que
/// no ha almacenado ningún dato, asegurándonos de recibir Bytes de longitud 0 en retorno.
#[test]
fn test_store() {
    // Aquí registramos el contrato DataStore en un entorno Soroban por defecto 
    // y construimos un cliente que se puede usar para invocar el contrato.
    let env = Env::default();
    let contract_id = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_id);

    // Aquí generamos dos usuarios de prueba, u1 y u2, que serán los invocadores
    // de las funciones del contrato.
    let u1 = env.accounts().generate();
    let u2 = env.accounts().generate();

    // Para nuestra cuenta u1, almacenamos la representación en Bytes de "Hello Soroban!" 
    // usando la función put() del contrato. Luego usamos la función get() del contrato para asegurarnos 
    // de recibir el valor esperado.
    client
        .with_source_account(&u1)
        .put(&bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // hex value for "Hello Soroban!"
    assert_eq!(
        client.get(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    // Antes de almacenar cualquier valor como la cuenta u2, verificamos para asegurarnos de que get()
    // devuelva 0 Bytes (es decir, la cuenta no tiene datos para obtener).
    assert_eq!(client.get(&u2).len(), 0);

    // Ahora, como u2, invocamos la función put(), almacenando la representación Bytes
    // de "Soroban Quest 2", afirmando que get() debería devolvérnoslo de vuelta.
    client
        .with_source_account(&u2)
        .put(&bytes![&env, 0x536f726f62616e2051756573742032]); // hex value for "Soroban Quest 2"
    assert_eq!(
        client.get(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    // Por supuesto, esperamos que los datos para u1 no hayan sido sobrescritos por
    // u2 al invocar la función put().
    assert_eq!(
        client.get(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_store_value_too_short() {
    // Aquí registramos el contrato DataStore en un entorno Soroban por defecto,
    // y construimos un cliente que se puede usar para invocar el contrato.
    let env = Env::default();
    let contract_id = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_id);

    // Estamos generando un único usuario de prueba, u1, que será el invocador de
    // la función put() del contrato.
    let u1 = env.accounts().generate();

    // Para nuestra cuenta u1, intentamos almacenar Bytes (0, 7) usando la
    // función put() del contrato. Nos detenemos allí, ya que solo esperamos
    // que el contrato se desespere con el argumento que es demasiado corto.
    client
        .with_source_account(&u1)
        .put(&bytes![&env, 0x007]);
}

/// Para las próximas pruebas, vamos a probar el comportamiento del contrato DataStore
/// cuando se invoca desde otro contrato. Así que, estamos creando un
/// contrato inteligente muy simple aquí, que podemos usar en ellos. Es bastante simple,
/// y solo existe como cliente para invocar las funciones put(), get() y get_self() del contrato principal.
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    // Esta función pasa nuestro argumento de data suministrado a la función
    // put() del contrato DataStore. Este es uno de los dos errores de pánico que estamos
    // probando.
    pub fn try_put(env: Env, contract_id: BytesN<32>, data: Bytes) {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.put(&data);
    }

    // Esta función invoca la función get_self() del contrato DataStore. Este es
    // el segundo error de pánico que estamos probando.
    pub fn try_get_s(env: Env, contract_id: BytesN<32>) -> Bytes {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.get_self()
    }

    // Esta función invoca la función get() del contrato DataStore,
    // pasando un argumento de owner que contiene un AccountId.
    pub fn try_get(env: Env, contract_id: BytesN<32>, owner: AccountId) -> Bytes {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.get(&owner)
    }
}

/// Este test intenta invocar el método put() del contrato DataStore, como
/// otro contrato inteligente. Se espera que esto falle ya que ese método solo está
/// disponible para una Cuenta, y no para un Contrato.
#[test]
#[should_panic(expected = "Status(ContractError(1))")] // We want this test to panic since it uses a forbidden function.
fn test_contract_store() {
    // Al igual que todos los tests de Soroban, creamos un entorno y registramos el
    // contrato DataStore en él.
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);

    // Tomamos un paso adicional para registrar nuestro contrato Caller en el entorno,
    // para que podamos probar las llamadas entre contratos usando su cliente.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // Invocamos la función put() del contrato DataStore usando la función
    // try_put() de nuestro contrato Caller. Esperamos que esto cause un pánico.
    caller_client.try_put(
        &contract_id_data_store,
        &bytes![&env, 0x48656c6c6f20536f726f62616e21],
    );
}

/// Este test intenta invocar el método get_self() del contrato DataStore,
/// como otro contrato inteligente. Se espera que esto falle ya que ese método solo está
/// disponible para una Cuenta, y no para un Contrato.
#[test]
#[should_panic(expected = "Status(ContractError(1))")] // We want this test to panic since it uses a forbidden function.
fn test_contract_get_self() {
    // Creamos un entorno y registramos el contrato DataStore en él.
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);

    // Tomamos un paso adicional para registrar nuestro contrato Caller en el entorno,
    // para que podamos probar las llamadas entre contratos usando su cliente.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // Invocamos la función get_self() del contrato DataStore usando
    // el método try_get_s() de nuestro contrato Caller. Esperamos que esto cause un pánico.
    caller_client.try_get_s(&contract_id_data_store);
}

/// Este test intenta invocar el método get() del contrato DataStore, como
/// otro contrato inteligente. NO se espera que esto cause un pánico ya que get() está
/// expuesto para ser invocado desde un Contrato.
#[test]
fn test_contract_get() {
    // Creamos un entorno y registramos el contrato DataStore en él. También
    // estamos creando un cliente para este contrato, para que podamos invocar la
    // función get() y esperar algunos datos reales (no Bytes(0)).
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);
    let client_data_store = DataStoreContractClient::new(&env, &contract_id_data_store);

    // Tomamos un paso extra para registrar nuestro contrato Caller en el entorno,
    // para que podamos probar las llamadas de contrato cruzado, utilizando su cliente.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // Creamos una Cuenta, u1, para que podamos invocar la función put() y
    // comparar contra el valor que almacenamos, cuando se llama desde nuestro contrato más tarde.
    let u1 = env.accounts().generate();
    client_data_store
        .with_source_account(&u1)
        .put(&bytes!(&env, 0x48656c6c6f20536f726f62616e21));

    // Invocamos la función get() del contrato DataStore mediante el método
    // try_get() desde el contrato Caller. No esperamos que esto cause una pánico,
    // ya que la función get() puede ser invocada por un Contrato. Sin embargo,
    // sí esperamos que nuestro valor devuelto coincida con el valor que put antes.
    let value = caller_client.try_get(&contract_id_data_store, &u1);
    assert_eq!(value, bytes!(&env, 0x48656c6c6f20536f726f62616e21));
}
