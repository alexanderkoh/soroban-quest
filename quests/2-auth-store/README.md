# Quest 2 - Tienda de autenticación

## TL;DR

¿Te consideras "avanzado"? ¿Crees que puedes saltarte la teoría de alto nivel
y escapar corriendo a esta aventura? ¿Te sientes cómodo
completamente arruinando tu cuenta antes de entender la tarea?

**En esta quest, desplegarás el contrato Quest 2 Auth Store en la Futurenet de Stellar. 
Luego, debes invocar con éxito la función `put()` para almacenar algún
datos en el ledger, y luego invocar con éxito la función `get()` o
`get_self()` para recuperar esos mismos datos.**

## Tabla de contenidos <!-- omit in toc -->

* [TL;DR](https://chat.openai.com/chat#tldr)
* [Cómo jugar](https://chat.openai.com/chat#c%C3%B3mo-jugar)
* [Un pequeño consejo](https://chat.openai.com/chat#un-peque%C3%B1o-consejo)
* [La tarea a mano](https://chat.openai.com/chat#la-tarea-a-mano)
  * [Explora el código del contrato](https://chat.openai.com/chat#explora-el-c%C3%B3digo-del-contrato)
  * [Almacenar datos](https://chat.openai.com/chat#almacenar-datos)
  * [Recuperar datos](https://chat.openai.com/chat#recuperar-datos)
  * [Autenticación simple](https://chat.openai.com/chat#autenticaci%C3%B3n-simple)
* [Lectura adicional](https://chat.openai.com/chat#lectura-adicional)
* [¿Todavía atascado?](https://chat.openai.com/chat#todav%C3%ADa-atascado)

## Cómo jugar

Si perdiste nuestra aventura anterior, o solo necesitas un repaso, tenemos
algunas (muy extensas) instrucciones sobre la *mecánica* de completar estas
aventuras (generar parejas de claves, verificar tu trabajo, etc.).

Toda esa información [se puede encontrar aquí][how-to-play] si necesitas usar esas
instrucciones de nuevo.

## Un pequeño consejo

En la última quest, varios usuarios tuvieron dificultades para desplegar o invocar
cualquier función en la red Futurenet, porque su punto final RPC todavía no había
completado el proceso de sincronización con los otros nodos pares de la red.

Aquí hay un pequeño consejo, si quieres verificar el estado de tu punto final RPC. Ejecuta este comando en tu terminal Futurenet:

```bash
curl -s https://horizon-futurenet.stellar.org/ | grep ingest_latest_ledger && curl -s http://127.0.0.1:8000 | grep ingest_latest_ledger
```

Deberías obtener una salida que se parezca a la siguiente:

```bash
gitpod /workspace/soroban-quest (main) $ curl -s https://horizon-futurenet.stellar.org/ | grep ingest_latest_ledger && curl -s http://127.0.0.1:8000 | grep ingest_latest_ledger
  "ingest_latest_ledger": 799299,
  "ingest_latest_ledger": 0,
```

La primera línea representa el último libro mayor en los nodos Futurenet mantenidos por SDF, y la segunda línea representa el último libro mayor en el espacio de trabajo de Gitpod. Si esos dos números están demasiado alejados, no podrás interactuar con Futurenet a través de tu punto final RPC local.

En ese caso, solo tendrás que esperar pacientemente mientras tu nodo Futurenet se sincroniza con el resto de la red.

## La tarea en cuestión

¡Así que, al grano! Vamos a descubrir cómo funciona esta misión. Un contrato Soroban puede almacenar datos arbitrarios en el libro mayor, y luego estos datos pueden ser recuperados más tarde. Tu trabajo hoy es almacenar y posteriormente recuperar datos invocando un par de funciones en tu contrato. (Después de que haya sido implementado en Futurenet, ¡por supuesto!)

### Explorar el código del contrato

Al igual que todas las misiones, los archivos [`lib.rs`](https://chat.openai.com/src/lib.rs) y [`test.rs`](https://chat.openai.com/src/test.rs) de esta misión están comentados con documentación y explicaciones útiles de lo que está sucediendo. Asegúrate de revisarlos y leer lo que el contrato está haciendo.

**Importante** : Esta misión tiene comentarios y documentación muy completos (especialmente en las pruebas) en esos dos archivos. Irá **millas** para ayudar a tu comprensión de Soroban, si lees a través de esos archivos y comprendes lo que está sucediendo.

### Almacenamiento de datos

Soroban utiliza la función `Env.data().set()` para almacenar datos en
las entradas de registro de un contrato. Puedes pensar en estas entradas de registro como
almacenamiento de clave-valor que solo se puede acceder a través del contrato que lo posee.
Se pueden construir las entradas de registro de un contrato de muchas maneras diferentes. Pueden
estar hechos de elementos muy simples como un símbolo o un número. O, también pueden
estar hechos de vectores o mapas muy complejos.

Las entradas de registro para esta misión almacenarán un `valor` suministrado junto con un
`AccountId`. El uso de la función `Env.invoker()` de Soroban nos brinda un método simple
de autenticación de un usuario. Solo alguien que pueda firmar para una cuenta (y, por lo tanto, invocar el contrato desde la cuenta) está permitido almacenar
datos en este contrato como esa cuenta.

*Invoca la función `put()` del contrato para almacenar algún tipo de datos en las entradas de registro del contrato.*


### Recuperación de datos

Para recuperar datos desde las entradas de registro de un contrato, está disponible para nosotros la función `Env.data().get()`. 
Cuando se llama con una `clave` que corresponde a datos que el contrato ha almacenado previamente, obtenemos el `valor` almacenado 
junto a él en retorno.

La función `get_self()` del contrato busca datos asociados con la cuenta del invocador. No es necesario pasar ningún argumento aquí, ya que sabemos buscar el invocador.

Por otro lado, la función `get()` recuperará los datos almacenados asociados con cualquier cuenta. Cuando se suministra un `AccountId` como argumento, esta
función buscará los datos almacenados correspondientes a esa cuenta. (Me pregunto si
esto puede tener algún impacto en el futuro?)

Para completar esta misión, solo tienes que invocar una de estas funciones. No tienes que intentar invocar ambas, aunque no hacer daño intentarlo!

*Invoca la función `get()` o `get_self()` del contrato para recuperar algún dato de contrato.*

### Autenticación Simple

Bueno, ¿cuál es el objetivo de todo esto? Por supuesto, es bastante interesante poder almacenar y recuperar datos de la red de contratos inteligentes. Pero, ¿hay algo más... "útil" sobre esto?!

Bueno, por supuesto! Para empezar, puedes controlar *quién* está permitido establecer *cuál* de las llaves de datos del contrato. De la manera en que hemos codificado este contrato, **sólo** una persona que puede firmar por una cuenta Stellar determinada (es decir, una clave pública) puede almacenar o modificar datos asociados con esa identificación de la cuenta. La función `Env.invoker()` puede ser una manera bastante rápida y simple de autenticar que un contrato está siendo invocado por alguien que tiene la autorización adecuada para invocar un contrato utilizando una cuenta determinada.

## Lectura Adicional

* Echa un vistazo al contrato de ejemplo de[almacenamiento de datos](https://soroban.stellar.org/docs/examples/storing-data) para una discusión adicional sobre este método de almacenamiento y recuperación de datos.
* Una discusión sobre métodos de autenticación más avanzados se puede encontrar en el contrato de ejemplo de [autenticación (avanzada)](https://soroban.stellar.org/docs/examples/auth-advanced).
* Puedes aprender más sobre [persistencia de datos](https://soroban.stellar.org/docs/learn/persisting-data) en la sección "Aprender"
  de la documentación de Soroban.

## Todavía Atascado?

Si te estás topando con un muro, y no estás seguro de tu próximo movimiento,
consulta [esta sección](https://chat.openai.com/README.md#feeling-lost) en nuestro README principal. Tiene
una o dos sugerencias de dónde ir a partir de aquí.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[data-example]: https://soroban.stellar.org/docs/examples/storing-data
[auth-advanced]: https://soroban.stellar.org/docs/examples/auth-advanced
[persist-data]: https://soroban.stellar.org/docs/learn/persisting-data
