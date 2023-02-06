# Quest 3 - Reverse Engineer <!-- omit in toc -->

## TL;DR

Tu nuevamente!? ¿Regresaste aquí buscando la tarea rápida? 
Bien, está bien, si crees que realmente estás listo para ello. ¡Buena suerte!

**Para esta misión, ya hemos desplegado el contrato ReverseEngineerContract 
usando la cuenta GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI. 
¡Debes encontrar el contratoId para este contrato! Luego, debe invocar la función correcta en ese contrato, proporcionando el argumento correcto.**


## Tabla de Contenidos <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Using Soroban-CLI to Decode XDR](#using-soroban-cli-to-decode-xdr)
  - [How do I find a `contractId`?](#how-do-i-find-a-contractid)
    - [Find an Operation](#find-an-operation)
    - [View the Transaction Result](#view-the-transaction-result)
    - [Sidenote About Reading Deployed WASM Binaries](#sidenote-about-reading-deployed-wasm-binaries)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

Si te perdiste de nuestras misiones anteriores, o necesitas un recordatorio, tenemos algunas 
instrucciones *mecánicas* (como generar parejas de claves, verificar tu trabajo, etc.) para completar estas misiones. 
Toda esa información está [aquí][how-to-play] si necesitas usar esas instrucciones nuevamente.

## The Task at Hand

Oigo que quieres: "¡Vamos a la aventura!" ¡Me encanta tu entusiasmo! 
Pero, quiero ~~suplicar~~ implorarte que hagas una cosa primero: ¡Lee el código!

### Explore the Contract Code

Ok, así que hemos estado en esto un par de veces antes. Puedes sentir que estás *empezando* a entender el orden de las cosas y cómo todo esto funciona. **O**, puedes sentir que estás totalmente perdido y simplemente quieres que alguien más te dé la respuesta.

Cualquiera que sea tu sentimiento, quiero poner esto en la voz más enfatizada que pueda: **¡Lee el código que está contenido dentro de `src/lib.rs` y `src/test.rs`!** Lee todo. ¡Todos los comentarios y todo! Todo.

Confía en mí, querrás leer el contrato para esto (todas las misiones, honestamente). Claro, leer el código te ayudará a entender el contrato. Sí, eso es lo que estamos tratando de hacer. **Pero**, también responderá muchas de las preguntas que tendrás mientras trabajas en esta tarea.

### Using Soroban-CLI to Decode XDR

Un término con el que puedes estar familiarizado es "XDR", que significa Representación de Datos Externos. 

Gran parte de lo que ocurre en la red Stellar se realiza en formato XDR: transacciones, datos del libro mayor, historial, 
resultados de operaciones y la lista sigue. XDR es un formato binario compacto y eficiente en la red. Si bien es excelente 
para muchas cosas, no es legible para humanos, por lo que puede ser bastante confuso.

Afortunadamente, la [Soroban CLI][soroban-cli] hace que sea bastante fácil 
obtener una salida decodificada, útil y comprensible de XDR proporcionado. Por ejemplo, cuando 
se envía una transacción a la Red, se envía en formato XDR. Aquí hay un ejemplo de cómo usar el 
comando `soroban` para decodificar una transacción de Friendbot XDR en un formato más legible para humanos.

```bash
soroban xdr dec --type TransactionEnvelope --xdr AAAAAgAAAABhi8yJmyMMTBza5emErFGm+xbj3PeggjF1g0CVlG+jOQAPQkAAAFRyAAAABgAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAABwLBiyzjjMVWCiToDYJPGrLhVy4+Ndx26l2x28bngMAwAAABdIdugAAAAAAAAAAAKUb6M5AAAAQDqxm6rPqDMypQWNAyZY17x5YG+cEuhZ8kqD868mMskTKE0jxD3lTY73gddFdmZZZ4Tit2pwQOtXI7w+M4OGoAiGVuCcAAAAQO0kX9PUv7sLyAaGvyw4l8NR86S5Pj9erMIOj0u2qmEgDW8YRQERFH3JkF/GX2B8NMy8NCd5/KWgj9b3iahk+QQ= --output json
```

Omitiré la salida aquí, en un intento de mantener este README sin desorden. 
Pero, si ejecutaras el comando anterior, verías una representación en formato JSON de una 
transacción que crea una cuenta con un saldo inicial de 10,000 XLM (por supuesto, futurnet XLM). 
Existen muchos diferentes "tipos" que puedes utilizar el cli de soroban para decodificar. 
Si ejecutas `soroban xdr dec --help` en tu terminal, verás todas las diferentes opciones que están disponibles para ti.

### How do I find a `contractId`?

¿Alguna vez has escuchado la expresión "Hay más de una manera de pelar un gato"? 
Sé, es desagradable y asqueroso! ¡No sé quién querría tener *una* manera de pelar un gato!! 
De todos modos, me estoy desviando ...

Existen algunas maneras diferentes de encontrar un `contractId` para un contrato ya desplegado. 
Todos ellos implican decodificar XDR y eso se puede hacer con soroban-cli, el Laboratorio Estelar, 
RunKit o podrías encontrar tu propia manera de decodificar / codificar la base64 según sea necesario. 
Pero todos comienzan encontrando el XDR correcto para decodificar.

La siguiente abordaje sensato para encontrar un 'contractId' comenzará encontrando una 'operación' 
relevante para la cuenta en cuestión, y luego la correspondiente 'transacción'. 
Esto definitivamente no es la única manera de hacerlo (ni siquiera es la más rápida o fácil), pero sí es directa y fácil de seguir.

#### **Find an Operation**

Comencemos dirigiéndonos al [Laboratorio Estelar][lab] (utilizando futurenet), y hacemos clic
en **Explore Endpoints** -> **Operations** -> **Operations for Account**. Introduzca
la clave pública de una cuenta existente en futurenet (el ejemplo aquí muestra una
cuenta que se usó para implementar y invocar el contrato `HelloWorld` desde la primera misión), y haga clic en **Aceptar**. (Si prefiere ver la salida JSON, puede obtenerla [aquí][ops]).

<details>
<summary>View screenshot</summary>

![Operations for Account](https://user-images.githubusercontent.com/2024293/202301202-4cc30e8e-b5ca-4efd-873d-52d75a43eb50.png)

</details>

**Recordatorio**: Estás buscando un contrato desplegado por esta dirección:
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

**Note**: You could also use this same technique to find some pretty useful
information from `HostFunctionHostFnInvokeContract` operations. You could use
those operations to see exactly what a given account used to invoke a given
contract. Check out [this video][twitch] to learn a bit more!

#### **View the Transaction Result**

From there, we find the link to the **transaction** that contains this
operation. It's provided in the operation's `_links.transaction` object. If
you're in the Lab, you can click on that link and it will open up the endpoint
explorer with the fields for that transaction pre-filled, and you just have to
click **Submit** once again. (For the JSON among us, you can copy/paste the link
into your browser, or you can [click here][tx].)

In the transaction information, you're looking for the `result_meta_xdr` field.
This contains the result from the transaction, as well as what has changed in
the networ as a result of the transaction. Most pertinent to this quest, it will
contain the `contractId` of the deployed contract. In the Lab, if you click on
that XDR string, it will take you to the XDR viewer, where you can find the
`contractId` (don't forget to [decode the base64][twitch-clip] somehow).

<details>
<summary>View screenshot</summary>

![Transaction Result Meta XDR](https://user-images.githubusercontent.com/2024293/202301714-082efbb5-7350-45ec-8a1a-a062ea8fe444.png)

</details>

Alternatively, you could copy/paste the whole Result Meta XDR string and decode
it using the Soroban CLI to get the information you're after.

#### **Sidenote About Reading Deployed WASM Binaries**

This is merely tangential to today's Quest, but it is very interesting and
useful nonetheless.

<details>
<summary>Are you curious? Go ahead. Read on...</summary>

The reason we've taken you to see the full transaction meta is to point out that
included in this XDR is also the `contractCode`! Yeah, that's right. The whole
thing! You could even decode it and use it as a normal WASM file, too. You'd
have to do it like this:

```bash
soroban xdr dec \
    --type TransactionMeta \
    --xdr <put your transaction_result_meta XDR here> \
    --output json

# look for the output field labeled `contractCode`. It's displayed in hex, so
# you'd have to convert it into a binary file, maybe like this:

echo "<the-wasm-hex>" | xxd -r -p > output.wasm
```

The `output.wasm` file resulting after that would be identical to the compiled
contract that was initially deployed. You could re-deploy it, use `soroban gen`
to get information about it, or whatever else you could come up with. Cool,
huh!?

Like we said, this isn't particularly important to this quest, but it could come
in handy at some point for you.
</details>

## Further Reading

- [Learn more about XDR][xdr] in the Stellar Developer Documentation.
- [This episode][twitch-full] of "Soroban Talks" is **SO** useful, and can help
  you get a handle on what's happening inside of Soroban. (Hint: Starting around
  [23:14][twitch] is a *really* useful discussion about decoding the XDR values
  into something a little more user-friendly.)
- Developers can also use the Soroban-RPC interface to interact with futurenet
  and get current state data. [This design doc][soroban-rpc] is being used
  discuss and develop how this API functions.
- Some basic information about the usage of the Soroban CLI can be found on the
  [Soroban Docs website][install-soroban]. In addition to that page many of the
  tutorials and examples contain example CLI commands.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

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
