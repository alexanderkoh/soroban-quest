# Quest 4 - Contrato cruzado <!-- omit in toc -->

## TL;DR

¿Otra vez tú!? ¿Vuelves aquí buscando la tarea rápida? Bueno, está bien, si crees
que realmente estás listo para ello. ¡Buena suerte!

**Para esta quest, usarás tu cuenta de Quest para desplegar el contrato
`DataStore` de la Quest 2, invocando la función `put` para almacenar algunos datos en
cadena. También debe usar su cuenta de Quest para desplegar el contrato
`CrossContractCall` de esta Quest, y usarlo para realizar una invocación de contrato cruzado de
la función `get` de su contrato `DataStore`.**

## Tabla de Contenidos <!-- omit in toc -->

* [TL;DR](https://chat.openai.com/chat#tldr)
* [Cómo jugar](https://chat.openai.com/chat#c%C3%B3mo-jugar)
* [La tarea a mano](https://chat.openai.com/chat#la-tarea-a-mano)

  * [Crear un Oráculo en Cadena](https://chat.openai.com/chat#crear-un-or%C3%A1culo-en-cadena)
  * [Importación de contratos](https://chat.openai.com/chat#importaci%C3%B3n-de-contratos)
  * [Usando un cliente de contrato](https://chat.openai.com/chat#usando-un-cliente-de-contrato)
  * [Pasando argumentos a Soroban CLI](https://chat.openai.com/chat#pasando-argumentos-a-soroban-cli)
* [Lectura adicional](https://chat.openai.com/chat#lectura-adicional)
* [Todavía atascado?](https://chat.openai.com/chat#todav%C3%ADa-atascado)

## Cómo jugar

Si te perdiste de nuestras quests anteriores, o solo necesitas un repaso, tenemos algunas instrucciones (bastante extensas) para las *mecánicas* de completar estas misiones (generación de pares de claves, revisión de tu trabajo, etc.).

Toda esa información [se puede encontrar aquí][cómo-jugar] si necesitas usar esas instrucciones de nuevo.

## La tarea a mano

Sabemos que estás emocionado y listo para empezar a hacer misiones! Pero, de nuevo, por favor, por favor
por favor: **¡Lee el código!!** Hay cosas importantes que necesitas saber
dentro de allí.

Ahora, hablemos de teoría:

### Creación de un Oráculo en la Cadena

Un "oráculo" de blockchain podría parecer una de esas palabras en tendencia que suenan como algo genial, pero no estás seguro de lo que significa realmente. Puedes pensar en un oráculo como una ventana al "mundo exterior" dentro de una cadena de bloques. Un oráculo trae datos del exterior para su uso en la red. ¡Podrías crear uno que recoja todo tipo de datos! Quizás se trate de:

* Datos climáticos de todo el mundo.
* Los registros actuales de velocidad para Super Mario.
* Resultados y clasificaciones de fútbol antes de la Copa del Mundo.
* El precio de Bitcoin contra otro activo.
* Tú entiendes... ¡podría ser prácticamente cualquier cosa!

Un oráculo también podría existir como una base de datos en la cadena. Quizás lo llenes con las direcciones públicas de tus amigos más cercanos (o tus enemigos). O, podrías almacenar tus recetas de guacamole delicioso. Ese dato luego está disponible para su uso en otros contratos inteligentes, donde quizás necesites ese dato para diferentes (¿nefastos?) propósitos. ¿Muy cool, verdad?

Quizás podrías reutilizar el contrato de la Quest 2 para que sea algún tipo de almacenamiento de datos en la cadena que contenga lo que quieras. Después de todo, hay una función `get()` que se puede invocar desde otros contratos.

### Importando contratos

Entonces, tienes un contrato que quieres invocar desde dentro del tuyo? Y, ¿quieres saber cómo se hace? Para invocar `contract_a` desde dentro de `contract_b`, primero debes importar el binario compilado de `contract_a` en tu código para `contract_b`. Al hacerlo, suceden un par de cosas dentro de `contract_b`:

* Cualquier tipo personalizado que se declare en`contract_a` ahora es utilizable en`contract_b`
* Se genera un`ContractClient` que se puede usar para invocar las funciones de`contract_a`

Así es cómo podría suceder esto en el archivo `contract_b/src/lib.rs`:

```rust
// We put this inside a `mod{}` block to avoid collisions between type names
mod contract_a {
    soroban_sdk::contractimport!(file = "contract_a.wasm");
}
```

**Nota**: Al importar un archivo de contrato en otro contrato, es un buen momento para pensar en si desea optimizar su proceso de compilación. Puede leer más sobre [Optimización de compilaciones][optimizando] en la documentación de Soroban.

### Advertencia sobre el orden de compilación de los contratos <!-- omit in toc -->

Anteriormente, a menudo utilizábamos un Makefile para automatizar algunas de las construcciones de contratos en esta serie de misiones. Esto a menudo podía resultar en errores que se arrojaran en tiempo de construcción para contratos que eran irrelevantes para lo que un usuario intentaba lograr.

Para facilitar la finalización de esta misión y minimizar la confusión, hemos precompilado e incluido el mismo contrato de la misión 2 dentro de este directorio.

En caso de que esté escribiendo su propia funcionalidad entre contratos, es importante que un archivo compilado ya exista cuando utiliza la macro `contractimport`. De lo contrario, su construcción fallará (y espectacularmente).

### Usando un cliente de contrato

Una vez que se ha importado "contract_a" en "contract_b", utilizar una llamada de contrato cruzado es muy sencillo. El proceso se ve así:

* `"contract_b"` crea un cliente que usará para invocar funciones en `"contract_a"`.
* `"contract_b"` realiza una invocación usando ese cliente y suministrando cualquier argumento que pueda ser necesario.
* `"contract_a"` ejecuta la función invocada y devuelve su respuesta a `"contract_b"`.
* `"contract_b"` luego toma la respuesta y hace lo que sea necesario con ella (devuelve toda o parte de la respuesta, procesa la respuesta y devuelve algo más, llama a otro contrato, tú lo entiendes).

Puedes pensar en este cliente de contrato como si fuera un módulo existente que estás usando en tu propio contrato. ¡No es tan malo, Soroban mi viejo amigo!

### Pasar argumentos a Soroban CLI

Recuerda la Quest 2 por un momento. Si fuiste una de las muchas personas, es posible que te encontraras con un contrato desplegado que no contenía la función `get_self()`. Después de mirar en confusión por unos momentos, es posible que comenzaras a pensar en cómo podría ser posible invocar esa función `get()`. Si lograste descubrir eso, bien hecho! No es una tarea obviamente inmediata... Así que, aprendamos un poco sobre el proceso.

En caso de que aún no lo hayas notado, Soroban depende de [Remote Procedure Call (RPC)][rpc-wiki] para pasar mensajes entre clientes y la red. RPC se utiliza por un cliente para solicitar que un servidor ejecute una función y cuáles argumentos debería suministrar. Luego, el servidor ejecuta la función y informa al cliente. La ventaja de este enfoque es que la función no tiene que existir o ejecutarse en el cliente. Aquí hay [un artículo ilustrado][rpc-gforg] en GeeksforGeeks que va mucho más en profundidad.

Específicamente, Soroban utiliza [JSON-RPC][jsonrpc] para pasar y leer mensajes entre clientes y servidores. Esto usa el formato de datos JSON para esos mensajes. Por lo tanto, en algunos casos, puedes pasar una cadena JSON como argumento a la función de un contrato. El ejemplo [Auth (Avanzado)][auth-advanced] en la documentación describe cómo puedes usar esta técnica para invocar un contrato y decirle al contrato que se ejecute usando la autenticación `invoker()`. El uso de la palabra "Invoker" en el ejemplo a continuación es una forma *fija* de autenticarse con el contrato de ejemplo. Esto significa que la autenticación `invoker()` no funcionaría si suministraras cualquier otra palabra, como `myPassword` o `AccountId`, por ejemplo.

soroban invoke
--wasm target/wasm32-unknown-unknown/release/soroban_auth_advanced_contract.wasm
--id 1
--account GC24I42QMKKR4NE6IYNPCQHUO4PXWXDGNZ7QVMMSR5EWAYSGKBHPLGHH
--fn increment
--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}'
--arg 0

Se puede ver que es claramente un objeto JSON que no se presenta de manera "bonita". Algunos otros ejemplos de estos argumentos JSON podrían verse así:

```json
'[{"u32":5}]' // the integer 5
'{"object":{"vec":[{"symbol":"<helloworld>"}]}}' // a single-element `Vec` containing the `Symbol` "Invoker"
'{"map":[{"key":{"symbol":"<key>"},"value":{"symbol" :"<value>"}}]}' // a `Map` with a key-value pair of {"<key>": "<value>"}
'{"contractCode":{"wasm":"<raw_wasm_hex_encoded>"}}' // the hex-encoded binary that makes up a contract
```

Entonces, así es cómo pasar algunos de los argumentos más avanzados a la Soroban CLI, pero debes descubrir cómo pasar el argumento correcto para esta quest. Nuestro propio @Smephite ha reunido una [guía increíble][smephite-guide] sobre los diferentes tipos de soroban y cómo usarlos en las invocaciones de contratos. ¡Léelo! ¡De verdad! Necesitarás alguna de la información en ese documento para completar esta quest.

## Lectura adicional

* El contrato de ejemplo de [Llamadas de contrato cruzado][ccc-example] en la documentación de Soroban tiene aún más detalles e indicios sobre este tema.
* Lee más sobre [`traits` en Rust][rust-traits] en La Referencia de Rust
* La sección "Aprender" de la documentación de Soroban tiene un artículo completo sobre [interactuar con contratos][interacting-contracts], ¡y definitivamente vale la pena leerlo!
* No exploramos los detalles finos de mantener los datos en la cadena en esta misión, ¡pero hay mucho más por aprender sobre esto! Por favor, consulte el artículo [persistencia de datos][persisting-data] en la documentación de Soroban.
* Esta [Guía simple de tipos de Soroban][smephite-guide] es un cambio de juego absoluto para interactuar con contratos inteligentes desde la CLI de Soroban.

## Aún atascado?

Si te encuentras en un callejón sin salida y no estás seguro de cuál es tu siguiente movimiento, consulta [esta sección](https://chat.openai.com/README.md#feeling-lost) en nuestro README principal. Tiene algunas sugerencias sobre hacia dónde ir desde aquí.
