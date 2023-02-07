# Quest 3 - Ingeniería inversa <!-- omit in toc -->

## Resumen

¿¡Tú otra vez!? ¿Regresaste aquí buscando la tarea a la rápida?
Está bien, si crees que realmente estás listo para ello... ¡Buena suerte!

**Para esta misión, ya hemos desplegado el contrato ReverseEngineerContract usando la cuenta GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI. ¡Debes encontrar el contractId para este contrato! Luego, debes invocar la función correcta en ese contrato, proporcionando el argumento correcto.**

## Tabla de Contenidos <!-- omit in toc -->

- [Resumen](#tldr)
- [Como Jugar](#como-jugar)
- [La tarea a mano](#la-tarea-a-mano)
  - [Explora el Código del Contrato](#explora-el-código-del-contrato)
  - [Usando Soroban-CLI para decodificar XDR](#usando-soroban-cli-para-decodificar-xdr)
  - [Como encuentro un `contractId`?](#como-encuentro-un-contractid)
    - [Encuentra una Operacion](#encuentra-una-operación)
    - [Vizualiza el resultado de la transacción](#vizualiza-el-resultado-de-la-transacción)
    - [Nota al margen sobre la lectura de binarios WASM implementados.](#nota-al-margen-sobre-la-lectura-de-binarios-wasm-implementados)
- [Lectura Adicional](#lectura-adicional)
- [¿Todavía atascado?](#todavía-atascado)

## Como Jugar

Si te perdiste alguna de nuestras misiones anteriores, o necesitas un recordatorio, tenemos algunas instrucciones *técnicas* (como generar parejas de claves, verificar tu trabajo, etc.) para completar estas misiones.
Toda esa información está [aquí][how-to-play] por si necesitas usar esas instrucciones nuevamente.

## La tarea a mano

Entiendo que quieres "¡Empecemos la misión!" ¡Me encanta tu entusiasmo!
Pero, quiero suplicar e implorarte que hagas una cosa primero: ¡Lee el código!

### Explora el Código del Contrato

Ok, hemos hecho esto un par de veces antes. Puedes sentir que estás *empezando* a entender el orden de las cosas y cómo todo esto funciona. **O talvez,** puedes sentir que estás totalmente perdido y simplemente quieres que alguien más te dé la respuesta.

Cualquiera que sea tu sentimiento, quiero poner esto en la voz más enfatizada que pueda: **¡Lee el código que está contenido dentro de `src/lib.rs` y `src/test.rs`!** Lee todo. ¡Todos los comentarios y todo! Todo.

Confía en mí, querrás leer el contrato para esto (el de todas las misiones, honestamente). Claro, leer el código te ayudará a entender el contrato. Sí, eso es lo que estamos tratando de hacer. **Pero**, también responderá muchas de las preguntas que tendrás mientras trabajas en esta tarea.

### Usando Soroban-CLI para decodificar XDR

Un término con el que puedes estar familiarizado es "XDR", que significa Representación de Datos Externos.

Gran parte de lo que ocurre en la red Stellar se realiza en formato XDR: transacciones, datos del ledger, historial, resultados de operaciones y la lista sigue. XDR es un formato binario compacto y eficiente en la red. Si bien es excelente para muchas cosas, no es legible para humanos, por lo que puede ser bastante confuso.

Afortunadamente, la [Soroban CLI][soroban-cli] hace que sea bastante fácil
obtener una salida decodificada, útil y comprensible de XDR proporcionado. Por ejemplo, cuando se envía una transacción a la Red, se envía en formato XDR. Aquí hay un ejemplo de cómo usar el
comando `soroban` para decodificar una transacción de Friendbot XDR en un formato más legible para humanos.

```bash
soroban xdr dec --type TransactionEnvelope --xdr AAAAAgAAAABhi8yJmyMMTBza5emErFGm+xbj3PeggjF1g0CVlG+jOQAPQkAAAFRyAAAABgAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAABwLBiyzjjMVWCiToDYJPGrLhVy4+Ndx26l2x28bngMAwAAABdIdugAAAAAAAAAAAKUb6M5AAAAQDqxm6rPqDMypQWNAyZY17x5YG+cEuhZ8kqD868mMskTKE0jxD3lTY73gddFdmZZZ4Tit2pwQOtXI7w+M4OGoAiGVuCcAAAAQO0kX9PUv7sLyAaGvyw4l8NR86S5Pj9erMIOj0u2qmEgDW8YRQERFH3JkF/GX2B8NMy8NCd5/KWgj9b3iahk+QQ= --output json
```

Omitiré el resultado, en un intento de mantener este README ordenado. Pero, si ejecutas el comando anterior, verías una representación en formato JSON de una transacción que crea una cuenta con un saldo inicial de 10,000 XLM (por supuesto, futurnet XLM).
Existen muchos "tipos" diferentes que puedes utilizar en el cli de soroban para decodificar.
Si ejecutas `soroban xdr dec --help` en tu terminal, verás todas las opciones distintas que están disponibles.

### Como encuentro un `contractId`?

¿Alguna vez has escuchado la expresión "Hay más de una manera de pelar un gato"? Sé, es desagradable y asqueroso! ¡No sé quién querría tener *una* manera de pelar un gato!! De todos modos, me estoy desviando...

Existen distintast maneras de encontrar un `contractId` para un contrato que ya desplegado. Todas ellas implican decodificar XDR y eso se puede hacer con soroban-cli, el Laboratorio Stellar, RunKit o podrías encontrar tu propia manera de decodificar / codificar la base64 según sea necesario. Pero cada manera comienza encontrando el XDR correcto a decodificar.

La siguiente manera sensata para encontrar un 'contractId' comienza al encontrar una 'operación' relevante para la cuenta en cuestión, y luego la 'transacción' correspondiente.
Esta es definitivamente la única manera de hacerlo (ni la más rápida o fácil), pero sí es directa y fácil de seguir.

#### **Encuentra una Operación**

Comencemos dirigiéndonos al [Laboratorio Stellar][lab] (utilizando futurenet), y hacemos click en **Explore Endpoints** -> **Operations** -> **Operations for Account**. Introduce la clave pública de una cuenta existente en futurenet (el ejemplo aquí muestra una cuenta que se usó para implementar y invocar el contrato `HelloWorld` en la primera misión), y haz click en **Aceptar**. (Si prefieres ver la respuesta en JSON, puedes obtenerla [aquí][ops]).

<details>
<summary>View screenshot</summary>

![Operations for Account](https://user-images.githubusercontent.com/2024293/202301202-4cc30e8e-b5ca-4efd-873d-52d75a43eb50.png)

</details>

**Recordatorio**: Estás buscando un contrato desplegado con esta dirección:
`GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI`.

Cuando aparezcan los resultados, buscaremos una operación del "tipo"
`invoke_host_function` y con un campo "función" de HostFunctionHostFnCreateContractWithSourceAccount (es decir,
esta operación es una cuenta cargando/implantando un contrato inteligente). Nuestra operación de ejemplo:

```json5
{
  "_links": {
    "self": {...},
    // This is the link to the transaction you want to use (again, this is only an example)
    "transaction": {
      "href": "https://horizon-futurenet.stellar.org/transactions/6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47"
    },
    "effects": {...},
    "succeeds": {...},
    "precedes": {...}
  },
  "id": "3607489060802561",
  "paging_token": "3607489060802561",
  "transaction_successful": true,
  "source_account": "GBYCYGFSZY4MYVLAUJHIBWBE6GVS4FLS4PRV3R3OUXNR3PDOPAGAGPGK",
  // We want to find an operation where (type === "invoke_host_function")
  "type": "invoke_host_function",
  "type_i": 24,
  "created_at": "2022-11-16T21:38:03Z",
  "transaction_hash": "6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47",
  "parameters": [
    {
      "value": "AAAABAAAAAEAAAAEAAABgABhc20BAAAAAQ8DYAF+AX5gAn5+AX5gAAACDQIBdgFfAAABdgE2AAEDAwIAAgUDAQAQBhkDfwFBgIDAAAt/AEGAgMAAC38AQYCAwAALBzEFBm1lbW9yeQIABWhlbGxvAAIBXwADCl9fZGF0YV9lbmQDAQtfX2hlYXBfYmFzZQMCCqABApoBAwF/AX4CfyOAgICAAEEgayIBJICAgIAAAkAgAEIPg0IJUg0AQgUQgICAgAAhAiABQRxqQQI2AgAgASAANwMQIAFCyY7H1RM3AwggAUEIaiEDQQEhBAJAA0AgBEEDRg0BIAEgBDYCGCAEQQFqIQQgAiADKQMAEIGAgIAAIQIgA0EIaiEDDAALCyABQSBqJICAgIAAIAIPCwAACwIACwAeEWNvbnRyYWN0ZW52bWV0YXYwAAAAAAAAAAAAAAAXADsOY29udHJhY3RzcGVjdjAAAAAAAAAABWhlbGxvAAAAAAAAAQAAAAJ0bwAAAAAABgAAAAEAAAPqAAAABg==",
      "type": "Obj"
    },
    {
      "value": "AAAABAAAAAEAAAAEAAAAIP0hkKG8/MVooXYMhSjPxLDr9m0fTdFoOnXSFj36OY2X",
      "type": "Obj"
    }
  ],
  // AND this operation should be where (function === "HostFunctionHostFnCreateContractWithSourceAccount")
  "function": "HostFunctionHostFnCreateContractWithSourceAccount",
  "footprint": "AAAAAAAAAAEAAAAGUun3Vzk8sIx5g6OSFP0G+TFcYMwLubcSywQrXO9DqzYAAAADAAAAAw=="
}
```

**Note**: Puedes usar esta misma técnica para obtener información útil sobre las operaciones `HostFunctionHostFnInvokeContract`. Puedes usar esas operaciones para ver exactamente qué cuenta se utilizó para invocar un contrato determinado. Echa un vistazo a [este video][twitch] para obtener un poco más de información.

#### **Vizualiza el resultado de la transacción**

Desde aquí, encontramos el enlace a la **transacción** que contiene esta operación. Se proporciona en el objeto `_links.transaction` de la operación. Si estás en el Laboratorio, puedes hacer click en ese enlace y se abrirá el explorador de endpoints con los campos para esa transacción completados previamente, y solo tienes que hacer click en **Send** una vez más. (Para los que esten usando JSON, puedes copiar / pegar el enlace en tu navegador, o puedes [hacer clic aquí][tx].)

En la información de la transacción, estás buscando el campo `result_meta_xdr`. Este contiene el resultado de la transacción, así como lo que ha cambiado en la red como resultado de esta transacción. Lo más pertinente para esta Quest, contendrá el `contractId` del contrato ya desplegado. En el Laboratorio, si haces click en esa cadena de XDR, te llevará al visor XDR, donde puedes encontrar el `contractId` (no olvides [descifrar la base64][twitch-clip] de alguna manera).

<details>
<summary>Ver pantallazos

![Transaction Result Meta XDR](https://user-images.githubusercontent.com/2024293/202301714-082efbb5-7350-45ec-8a1a-a062ea8fe444.png)

</details>

Alternativamente, podrías copiar / pegar la cadena XDR Result Meta completa y decodificarla usando la CLI de Soroban para obtener la información que buscas.

#### **Nota al margen sobre la lectura de binarios WASM implementados.**

Esto es una tangente a la Quest de hoy, pero es muy interesante y útil de todos modos.

<details>
<summary>¿Curioso?</summary>

La razón por la que te hemos llevado a ver la metatransacción
completa es para señalar que está incluida en este XDR y también en el código del contrato. ¡Correcto! ¡Todo! Incluso podrías decodificarlo y usarlo como un archivo WASM normal. Tendrías que hacerlo así.:

```bash
soroban xdr dec \
    --type TransactionMeta \
    --xdr <put your transaction_result_meta XDR here> \
    --output json

# Busca el campo de salida etiquetado como "contractCode". Se muestra en hexadecimal, por lo que
# Tienes que convertirlo en un archivo binario, tal vez así:

echo "<the-wasm-hex>" | xxd -r -p > output.wasm
```

El resultado `output.wasm` será idéntico al contrato compilado que se desplegó inicialmente.
Puedes desplegarlo de nuevo, usar `soroban gen` para obtener información sobre él o cualquier otra cosa que puedas imaginar. ¡Genial, ¿no?

Como dijimos, esto no es particularmente importante para esta Quest, pero podría ser útil en algún punto para ti.

</details>

## Lectura Adicional

- Aprende más sobre XDR en la Documentación para Desarrolladores de Stellar.
- Este episodio de "Soroban Talks" es **TAN** útil y puede ayudarte a comprender lo que está sucediendo dentro de Soroban. (Pista: A partir de [23:14][twitch] hay una discusión *realmente* útil sobre la decodificación de los valores XDR en algo un poco más amigable para el usuario).
- Los desarrolladores también pueden usar la interfaz de RPC de Soroban para interactuar con futurenet y obtener datos del estado actual de la red. [Este documento de diseño][soroban-rpc] se está usando para discutir y desarrollar la forma en que funciona esta API.
- Puede encontrar alguna información básica sobre el uso de la interfaz de línea de comandos (CLI) de Soroban en el sitio web de Documentación de Soroban. [Esta página][install-soroban]. Además de esta página, muchos de los tutoriales y ejemplos contienen comandos de CLI de ejemplo.- Aprende más sobre XDR en la Documentación para Desarrolladores de Stellar.
- Este episodio de "Soroban Talks" es **TAN** útil y puede ayudarte a comprender lo que está sucediendo dentro de Soroban. (Pista: A partir de [23:14][twitch] hay una discusión *realmente* útil sobre la decodificación de los valores XDR en algo un poco más amigable para el usuario).
- Los desarrolladores también pueden usar la interfaz de RPC de Soroban para interactuar con futurenet y obtener datos del estado actual. [Este documento de diseño][soroban-rpc] se está usando para discutir y desarrollar la forma en que funciona esta API.
- Puede encontrar alguna información básica sobre el uso de la interfaz de línea de comandos (CLI) de Soroban en el sitio web de Documentación de Soroban. [Esta página][install-soroban]. Además de esta página, muchos de los tutoriales y ejemplos contienen comandos de CLI de ejemplo.

## ¿Todavía atascado?

Si estás golpeando una pared de ladrillos y no estás seguro de cuál es tu próximo paso, revisa esta sección en nuestro README principal.
Tiene un par de sugerencias para dónde podrías ir a partir de aquí.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[xdr]: https://developers.stellar.org/docs/encyclopedia/xdr
[soroban-cli]: https://github.com/stellar/soroban-cli
[lab]: https://laboratory.stellar.org
[ops]: https://horizon-futurenet.stellar.org/accounts/GBYCYGFSZY4MYVLAUJHIBWBE6GVS4FLS4PRV3R3OUXNR3PDOPAGAGPGK/operations?order=asc
[tx]: https://horizon-futurenet.stellar.org/transactions/6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47
[twitch]: https://www.twitch.tv/videos/1642865389?t=00h23m14s
[twitch-clip]: https://clips.twitch.tv/FragileSneakyOstrichGivePLZ-DK9h3VVmUjqVDDZG
[twitch-full]: https://www.twitch.tv/videos/1642865389
[soroban-rpc]: https://docs.google.com/document/d/1TZUDgo_3zPz7TiPMMHVW_mtogjLyPL0plvzGMsxSz6A/edit#heading=h.ohr0vgpzoi7r
[install-soroban]: https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli
