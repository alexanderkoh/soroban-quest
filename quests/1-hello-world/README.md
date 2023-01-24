# Quest 1 - Hello World <!-- omit in toc -->

## Sumario

Sé que estás apurado. ¡Hay Lumens en juego! Aquí te dejo la versión rápida y sucia de lo que necesitas hacer para conseguir la medalla SQ.

En esta misión, usarás tu **Cuenta Quest** para desplegar e invocar el contrato " 1 Hello World" en la Futurenet de Stellar.

## Tabla de Contenidos <!-- omit in toc -->

- [Sumario](#Sumario)
- [Cómo jugar](#como-jugar)
  - [Autenticarse con Discord](#authenticate-with-discord)
  - [Pull Nuevas Misiones (Quests)](#pull-in-new-quests)
  - [Recupera tu cuenta de misión](#retrieve-your-quest-account)
  - [Financiar tu cuenta](#fund-that-account)
  - [Empieza la misión](#quest-your-heart-out)
  - [Revisar tu respuesta](#check-your-quest-answer)
  - [Reclamar tu medalla](#claim-your-badge)
- [La Tarea a Mano](#the-task-at-hand)
  - [Explorar el código del contrato](#explore-the-contract-code)
  - [Construir el contrato](#build-the-contract)
  - [Ejecutar una prueba](#run-a-test)
  - [Desplegar en Futurenet](#deploy-to-futurenet)
  - [Invócalo en Futurenet](#invoke-it-on-futurenet)
- [Termina la misión](#finish-the-quest)
- [Lectura adicional](#further-reading)
- [¿Sigues Atascado?](#still-stuck)

## ¿Cómo Jugar?

Antes de que podamos empezar con las misiones, necesitamos asegurarnos de que realmente estás listo para empezar. ¡Haz lo siguiente antes de continuar!

**Nota**: Tendrás que seguir este procedimiento básico para cada una de las misiones de esta serie. Por favor, tómate un momento para revisar las siguientes instrucciones y familiarizarte con ellas. ¡Estarás agradecido que lo hiciste!

### Autenticarse con Discord

En uno de los terminales de  `bash` en el panel inferior de tu espacio de trabajo en Gitpod, ejecuta el siguiente comando:

```bash
sq user
```

Si ves que te has autenticado con éxito, puedes seguir adelante. De lo contrario, deberías ejecutar:

```bash
sq login
```

para iniciar sesión con tu cuenta de Discord. Luego se te pedirá que completes el KYC o envíes información fiscal, si es necesario.

**Nota**: Si no has completado el flujo de KYC y el envío de impuestos en el sitio de Stellar Quest, tu capacidad para reclamar recompensas XLM se verá afectada. Completa esos pasos para que podamos darte las recompensas por las que has trabajado tan duro.

### Pull Nuevas Misiones (Quests)

Cuando el contador llegue a 0, puedes importar la Quest recién publicada ejecutando el siguiente comando:

```bash
cd /workspace/<the-workspace-name> # `sq pull` must be run from inside the workspace root
sq pull
```

También puedes hacerlo manualmente, si eres un experto en `git`. Encuentra los detalles específicos en nuestra [Pioneer Quest][pq-new-quests].

Una vez completes esto, verás las nuevas misiones en el siguiente directorio:
`/workspace/<nombre-de-tu-espacio-de-trabajo>/quests/`

### Recuperar tu cuenta de misión

Antes de poder jugar, debes recuperar la `clave de misión` para la misión que deseas jugar. Puedes obtener esa información ejecutando lo siguiente:

```bash
sq play -i 1 # use whichever quest number you are trying to play
```

Guarda esta información, porque (créeme) la necesitarás más tarde.

### Financiar tu cuenta

Incluso armamos una manera práctica de obtener tus Lumens de Futurenet de
Futurenet Friendbot (quizás conozcas a su primo...). `sq` puede ayudarte con eso
de esta manera:

```bash
sq fund --key GDGYB5FZUKAVPYGCLJTCYYOJPEHHVOCZS7I6SBWF233OQSIROZ7JXLGO
```

### Empezar la misión

¡Ahora estás listo para pasar a la parte de la misión real de esto! Por favor, salta a [esta sección](https://chat.openai.com/chat#the-task-at-hand) para comenzar la parte divertida.

Cuando creas que has terminado todo lo necesario, regresa aquí y revisa tu trabajo.

### Revisar tu respuesta

Has hecho el trabajo duro y estás listo para ver si ha valido la pena. Ejecuta el
siguiente comando para verificar tu trabajo:

```bash
sq check -i 1 # usa el número de misión que estás tratando de verificar
```

Si todavía no has completado el KYC y/o el envío de impuestos, se te recordará una vez más que no puedes reclamar recompensas XLM sin ellos. Si todavía tienes más trabajo que hacer, se te dará una pista sobre qué puede faltar en tu cuenta de misión.

### Reclamar tu medalla

Si tu verificación fue exitosa, ¡la interfaz de línea de comandos `sq` te lo hará saber con un emoticono celebratorio! Entonces podrás elegir cómo quieres reclamar tu premio: firmar una transacción utilizando **Albedo** , o firmar el **Raw XDR** .

Si seleccionas "Albedo", se abrirá una ventana pidiéndote que firmes la transacción, permitiéndote así reclamar tu distintivo y cualquier XLM premiado que hayas ganado.

Si eliges "Raw XDR", la transacción se mostrará en la ventana del terminal y tendrás que firmarla usando Stellar Laboratory (o tu método preferido). Luego, deberás enviar la transacción firmada XDR utilizando el comando `sq CLI`. Sería algo así:

```bash
sq submit --xdr AAAAAgAAAADQTypLJCls2UK4wzQpHyTOdkEBKb78PvEFf7/UqD0P4gAPQkAACutwAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAACgAAAAdTdGVsbGFyAAAAAAEAAAAMUXVlc3QgUnVsZXMhAAAAAAAAAAGoPQ/iAAAAQGsvY+U2M6kGvsbJ+82A8lAQbZG/upocKynAFvADJETNUSbzMtG51KyQdetujsswz9rDDnjPosfZDVsq3ibUcAg=
```

¡Luego podrás darte una palmada en la espalda y disfrutar de tu increíble nuevo distintivo de Quest Stellar!

## La tarea a mano

Esta primera misión es bastante sencilla y viene directamente de los ejemplos de Soroban. ¡Sólo estamos mojando los pies, por lo que no tendrás que preocuparte por escribir contratos Rust o algo por el estilo! Aquí te presentamos lo que necesitas saber para completar esta misión.

### Explorar el código del contrato

Si abres el archivo [`lib.rs`](https://chat.openai.com/src/lib.rs), podrás ver algunos comentarios útiles que describen brevemente lo que hace cada parte del código.

*Este contrato acepta un argumento y responde con un saludo que contiene ese argumento proporcionado.*

Una dissecación más profunda de este ejemplo de contrato inteligente se puede encontrar en nuestra [Pioneer Quest][pq-lib].


### Construir el contrato

¡Ahora podemos pasar a construir realmente nuestro contrato! Sé que no pensabas llegar tan lejos. ¡Dáte una palmada en la espalda!

*El proceso de construcción compila nuestro código Rust que está diseñado específicamente para el entorno WebAssembly que Soroban proporcionará.*

Si necesitas algunas instrucciones para ayudarte en este paso, puedes consultar el [tutorial de construcción][docs-build] en la documentación de Soroban.

Aquí te presentamos la versión corta de lo que necesitas para construir el contrato. Ejecuta estos comandos desde tu terminal Futurenet:

```bash
# entra al directorio de quests
cd quests/1-hello-world
# construye el contrato
cargo build \
    --target wasm32-unknown-unknown \
    --release
```

> *Nota* : Solíamos enviar un `Makefile` con este repositorio que podía ejecutar este comando de construcción para cada misión. Esto introducía ciertos errores con misiones incompletas y presentaba a los usuarios con errores relacionados con misiones irrelevantes. Ya no enviamos ese `Makefile` y alentamos a los usuarios a construir manualmente cada contrato.

### Ejecutar una prueba

También puedes mirar el archivo [`test.rs`](https://chat.openai.com/src/test.rs) y ver que también tiene algunos comentarios útiles.

*Esta prueba invoca la función `hello()` del contrato y verifica que el contrato dé la respuesta esperada.*

Una vez que tengas una comprensión de lo que está sucediendo en el escenario de prueba, adelante y ejecuta la prueba, asegurando que nuestro contrato se esté comportando correctamente.

Al igual que antes, el [Pioneer Quest][pq-test] contiene una explicación mucho más elaborada de este archivo y de lo que está sucediendo en cada paso del camino.

Para ejecutar realmente la prueba en tu espacio de trabajo Gitpod de la misión, ejecuta estos comandos desde tu terminal de Futurenet:

```bash
# cambiate al directorio de quest (dado que no estés ahí ahora)
cd quests/1-hello-world
# ejecuta los tests en ese directorio
cargo test
```

> *Nota* : Solíamos enviar un `Makefile` con este repositorio que podía ejecutar este comando de prueba para cada misión. Esto introducía ciertos errores con misiones incompletas y presentaba a los usuarios con errores relacionados con misiones irrelevantes. Ya no enviamos ese `Makefile` y alentamos a los usuarios a probar manualmente cada contrato.

### Desplegar en Futurenet

El Stellar Futurenet es un lugar seguro donde tu código de contrato puede vivir y trabajar mientras aún está en desarrollo y (potencialmente) inestable.

*Desplegar el contrato subirá el archivo binario de construcción a Futurenet, haciéndolo disponible de inmediato para la invocación y el uso en el entorno Soroban de la red.*

Puedes encontrar algunas orientaciones sobre cómo desplegar tu contrato en Futurenet en el tutorial de [despliegue][docs-deploy] de la documentación de Soroban.

### Invócalo en Futurenet

Ahora llega la parte emocionante! Tu trabajo es **invocar** el contrato inteligente que acabas de subir a Futurenet. ¿No te da ganas de gritar de emoción?

*Invocar tu contrato usará la red Stellar para llamar al contrato que acabas de subir y ejecutar la función `hello()`.*

El tutorial de [despliegue][docs-deploy] también puede brindarte algunas orientaciones sobre los puntos finos de la invocación de un contrato Futurenet, si es necesario.

## Finalizar la misión

Si has llegado hasta aquí y no has estallado nada, probablemente estás en el camino correcto. Ahora es un buen momento para usar el CLI `sq` para revisar tu trabajo y tratar de reclamar tu premio.

Puedes encontrar esas [instrucciones aquí](https://chat.openai.com/chat#check-your-quest-answer).

## Lectura adicional

Ahora que has completado tu misión, es posible que tu interés esté despertado y estés curioso sobre todo tipo de cosas. Aquí tienes una recopilación de algunos recursos relacionados que te pueden interesar:

* El sitio web oficial de [Soroban] contiene anuncios, más recursos, oportunidades emocionantes de aprendizaje y más.
* El [Soroban CAP][cap] (y los relacionados) describen las opciones de diseño e implementaciones de la plataforma de contrato inteligente Soroban. (Advertencia: estos documentos pueden ser bastante técnicos).
* La documentación de Soroban contiene una serie de [contratos de ejemplo][examples] que pueden ayudarte a comprender mejor los conceptos y las ideas en las que se basa Soroban.
* Puedes aprender más sobre el [ciclo de vida del contrato][contract-lifecycle] en la sección "Aprender" de la documentación de Soroban. ¡Hay toneladas de excelentes artículos aquí y debes leer tantos como puedas!

## ¿Sigues atascado?

Si te estás topando con un muro y no estás seguro de cuál es tu próximo movimiento, echa un vistazo a [esta sección](https://chat.openai.com/README.md#feeling-lost) en nuestro README principal. Tiene un par de sugerencias sobre adónde ir a continuación.

[pq-lib]: https://github.com/tyvdh/soroban-quest--pioneer/blob/main/quests/0-hello-world/src/lib.rs
[pq-test]: https://github.com/tyvdh/soroban-quest--pioneer/blob/main/quests/0-hello-world/src/test.rs
[pq-new-quests]: https://github.com/tyvdh/soroban-quest--pioneer#getting-new-quests
[docs-build]: https://soroban.stellar.org/docs/tutorials/build
[docs-deploy]: https://soroban.stellar.org/docs/tutorials/deploy-to-futurenet
[soroban]: https://soroban.stellar.org
[cap]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046.md
[examples]: https://soroban.stellar.org/docs/category/examples
[contract-lifecycle]: https://soroban.stellar.org/docs/learn/contract-lifecycle
[sq-site]: https://quest.stellar.org/
