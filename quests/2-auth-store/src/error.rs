use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Necesitas ser un usuario para invocar esta función, las llamadas de "crosscontract" solo están permitidas para llamar la función get
    CrossContractCallProhibited = 1,
    // El valor del input es muy corto, usa al menos 11 caracteres (bytes)
    InputValueTooShort = 2,
}
