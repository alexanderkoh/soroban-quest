# Quest 6 - Interoperabilidad de activos <!-- omit in toc -->

## TL;DR

¡Gran día, ¿verdad! ¡La última misión en nuestra serie inaugural de Soroban Quest! Ha sido genial ¿verdad? Y solo habrá más grandeza por venir, así que mantente atento.

Si buscas la siguiente cosa, asegúrate de revisar y contribuir en [Sorobanathon].

¡La misión de hoy es impactante! No solo te desafiará, sino que también te mostrará algunas cosas **increíbles** que son posibles en este nuevo y valiente mundo Soroban-ificado. Ahora, es un poco difícil, por lo que realmente querrás leer a través de este documento. Pero, aquí están las instrucciones cortas y dulces, si quieres saltarte y arruinar las cosas.

*Hay dos cuentas relevantes hoy:

* `Parent_Account` será su Cuenta de Búsqueda, (lo que se le da cuando ejecuta`sq play {n}`) y se usará para desplegar un contrato`AllowanceContract`.
* `Child_Account` será una cuenta secundaria que interactuará con
  su contrato. Cree y financie esta cuenta de Futurenet por su cuenta.

**Para nuestra búsqueda final, debe construir y desplegar el contrato
`AllowanceContract` utilizando su Cuenta de Búsqueda (`Parent_Account`). Luego, use esa misma cuenta
para importar XLM de "Classic" Stellar a Soroban. A continuación, apruebe su contrato desplegado para actuar como un proxy que le permita transferir XLM del padre al
niño. Luego, utilizando cualquiera de las cuentas, debe retirar una asignación al
`Child_Account` usando el contrato desplegado por `Parent_Account`. Finalmente,
exporta esa asignación de nuevo a la cuenta de "Classic" Stellar de `Child_Account`.**

## Tabla de contenido <!-- omitir en la tabla de contenido -->

* [TL;DR](https://chat.openai.com/chat#tldr)
* [Cómo jugar](https://chat.openai.com/chat#c%C3%B3mo-jugar)
* [La tarea a mano](https://chat.openai.com/chat#la-tarea-a-mano)
  * [Activos &#34;Classic&#34; Stellar vs. Tokens &#34;Inteligentes&#34; Soroban](https://chat.openai.com/chat#activos-classic-stellar-vs-tokens-inteligentes-soroban)
  * [El contrato incorporado de Tokens](https://chat.openai.com/chat#el-contrato-incorporado-de-tokens)
  * [Sí, pero ¿cómo uso ese contrato incorporado de Tokens?](https://chat.openai.com/chat#s%C3%AD-pero-c%C3%B3mo-uso-ese-contrato-incorporado-de-tokens)
  * [XLM nativo en Soroban](https://chat.openai.com/chat#xlm-nativo-en-soroban)
  * [Vuelta a tu misión](https://chat.openai.com/chat#vuelta-a-tu-misi%C3%B3n)
* [Lectura adicional](https://chat.openai.com/chat#lectura-adicional)
* [¿Sigues atascado?](https://chat.openai.com/chat#sigues-atascado)

## Cómo jugar

Si te perdiste de nuestras misiones anteriores o solo necesitas un repaso, tenemos
instrucciones (bastante extensas) para las *mecánicas* de completar estas
misiones (generar claves, revisar tu trabajo, etc.).

Toda esta información [se puede encontrar aquí][como-jugar] si necesitas esas
instrucciones nuevamente.

## La tarea a realizar

Por un momento, ponte en la posición de un padre que envía a su hijo al mundo para enfrentar los desafíos de la universidad y/o el trabajo. Lo sé, es emocionante. Eso está bien. No quieres enviarlos sin nada. Quieres darles algo de confianza de que no tendrán un estómago vacío al final del día, sin importar lo que suceda.

Entonces, estás creando un contrato de asignación para ellos. Desplegarás un contrato que será una acción única por tu parte, pero que les estará disponible un flujo constante de recursos en caso de que alguna vez los necesiten. Este enfoque nos da dos victorias muy poderosas: *ellos* no pueden retirar todo de una sola vez, pero *tú* no tienes que recordar hacer transferencias todo el tiempo.

De vuelta en la realidad, estamos listos para hablar de activos! Pero, primero, tenemos que decirlo una última vez, **leer el código!** Este contrato es relativamente complejo. No te haré manipularlo hoy, pero no hay mejor manera de entender lo que está sucediendo que leerlo realmente. En serio.

### "Classic" Stellar Assets vs. Soroban "Smart" Tokens

> _Please Note_: If you've forgotten, Soroban is still under active development,
> design, and discussion. Significant changes can happen and should even be
> expected. The area of asset interoperability between "Classic" Stellar and
> Soroban "Smart" Tokens is one such area that is under active consideration. We
> have designed this quest to be as up-to-date as possible, but the conventions,
> steps, terminology, architecture, etc. used in today's quest are subject to
> change in the future.

Uno de las características definitorias de la red "Clásica" de Stellar es que los activos son un ciudadano de primera clase. Son fáciles de crear, baratos de usar/transferir/comerciar y útiles para muchos usos. También existe un extenso conjunto de herramientas de autorización que los emisores de activos pueden usar para controlar quién puede adquirir, usar o retener esos activos. No pasaremos demasiado tiempo aquí, porque probablemente ya estés al día aquí. Si necesitas un repaso, la [documentación para desarrolladores][docs-assets] y nuestro propio curso [Stellar Quest Learn][sq-learn] tienen **mucha** información sobre activos. Por ahora, solo recuerda que el activo `nativo` en Stellar (tanto "Clásico" como Soroban) es el [Lumen][lumens]. Se identifica usando el código de activo `XLM`.

A medida que continúa el desarrollo de Soroban, uno de los requisitos clave es que los activos emitidos en "Stellar Clásico" puedan ser utilizados e incorporados en Soroban. ¡Es incluso [una de las preguntas frecuentes][assets-faq]! Esta interoperabilidad se facilita mediante el uso de las funciones `importar` y `exportar` que existen en [el contrato de token incorporado](https://chat.openai.com/chat#the-built-in-token-contract), lo que permite que los activos emitidos en "Stellar Clásico" entren en juego en Soroban. (Tenga en cuenta que los activos acuñados en Soroban no se pueden exportar a un activo de "Stellar Clásico").

### El contrato de token incorporado

El desarrollo de Soroban en cuanto a activos implica un esfuerzo por decidir cómo se ve un "activo estandarizado" en un contexto de contrato inteligente. Estas decisiones y discusiones relacionadas se registran en [CAP-0046-06][cap-46-6]. Si estás familiarizado con Ethereum, esta propuesta intenta seguir un modelo ERC-20 donde sea aplicable.

El [contrato de token incorporado][token-contract] es una implementación de la propuesta CAP-46-6. Se puede utilizar para crear un nuevo token en Soroban o para envolver un activo "Clásico" y transferirlo a Soroban. Hoy, lo usaremos para envolver e importar Lumens desde "Stellar Clásico". Este contrato de token incorporado implementa una [interfaz de token][built-in-interface] que es bastante completa. Las funciones más importantes que necesitará hoy son `importar`, `aprobar` y `exportar`.

### ¡Sí, pero ¿cómo uso ese contrato de token incorporado?

> Hay que tener en cuenta que un desarrollador de tokens Soroban puede elegir implementar cualquier
> interfaz que elijan. No hay ninguna *requisito* de implementar todo
> desde CAP-46-6, pero hacerlo permite que un token sea interoperable con otros
> tokens que *sí* cumplen con CAP-46-6. Puedes aprender más sobre la
> [interfaz de token sugerida][token-interface] en los documentos de Soroban.

Entonces, ¿cómo realmente hacemos uno de esos tokens? Hay varios métodos
disponibles para nosotros. Echemos un vistazo (brevemente) a ellos.

1. (Picante 🌶️🌶️🌶️) Podrías escribir todo desde cero, implementando
   cualquier característica, función y diversión que satisfagan tus necesidades. Eso tomaría mucho
   trabajo, pero podrías hacerlo. No te lo impediré.
2. (Medio 🌶️🌶️) Hay un script`create.py` en el directorio`py-scripts/` aquí
   que hará mucho del trabajo pesado para ti. Esto se puede usar y adaptar
   para adaptarse al activo que estás intentando crear. Es un fantástico
   punto de partida.
3. (Suave 🌶️) Hecho divertido, la CLI de Soroban tiene una pequeña ayuda incorporada en sí misma que (te prometemos, no estamos haciendo esto) hará*todo* por ti. No tienes que codificar nada, solo ejecuta el comando una
   vez, y el contrato está**desplegado** . Lo puedes usar así:

```bash
soroban token wrap --asset QUEST6:GBCXQUEPSEGIKXLYODHKMZD7YMTZ4IUY3BYPRZL4D5MSJZHHE7HG6RWR

output:
42d792eb17c983b62bfac05fc31f9588675efd65867f26c56bafb2b15adb6e04

# It even works with the `native` asset!
soroban token wrap --asset native
```

Se debe tener en cuenta que el uso de la CLI de Soroban para implementar un activo envolvente funcionará solo una vez por activo (por red). Por lo tanto, el activo nativo ya está implementado en Futurenet, y intentar envolverlo de nuevo (en Futurenet) devolverá un error en lugar de una `contractId`.

> También se debe señalar que no es necesario implementar ni envolver ningún token ni activo para esta misión. Solo lo ponemos aquí por diversión!

### XLM nativo en Soroban

Hablando del activo "nativo": Una de las cosas interesantes sobre el contrato de token incorporado de Soroban es que incluso el token XLM nativo lo utiliza para llevar XLM a Soroban. Para usarlo, solo necesitamos descubrir el "contractId" que debemos invocar. Esto se puede hacer fácilmente con uno de los SDKs de Stellar (a continuación, estamos usando Python)

```python
import hashlib
from stellar_sdk import Asset, xdr

# This will work using either native or issued assets
native_asset = Asset.native()
issued_asset = Asset("QUEST6", "GBCXQUEPSEGIKXLYODHKMZD7YMTZ4IUY3BYPRZL4D5MSJZHHE7HG6RWR")

data = xdr.HashIDPreimage(
    xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID_FROM_ASSET,
    from_asset=native_asset.to_xdr_object(),
)
contract_id = hashlib.sha256(data.to_xdr_bytes()).hexdigest()
print(f"Contract ID: {contract_id}")
```

Una versión expandida del script anterior, así como algunos otros scripts de Python muy útiles (un gran grito a [Jun Luo (@overcat)][overcat]) en el directorio `py-scripts /`. Se ocupan de todo tipo de tareas de tokens Soroban: importación / exportación de saldos, creación de un token "Clásico" envolvente, encontrar identificadores de contrato, etc.

Según nuestro tl; dr en la parte superior, este contrato deberá invocarse al menos tres veces:

1. La cuenta`Parent_Account` necesitará`importar` el activo nativo.
2. La cuenta`Parent_Account` deberá`aprobar` el`AllowanceContract` como un pagador proxy.
3. La cuenta`Child_Account` (eventualmente) necesitará`exportar` su asignación de nuevo a Stellar "Clásico".

No te olvides de investigar la [Interfaz de token incorporada][built-in-interface] para averiguar qué argumentos debes usar al hacer esas invocaciones. ¿Recuerdas cómo formatear esos argumentos, verdad? ¿Qué!? ¿No lo haces?! Bueno, bueno, bueno. Todo estará bien. Vuelve a [Quest 4](https://chat.openai.com/4-cross-contract/README.md) y [Quest 5](https://chat.openai.com/5-custom-types/README.md) para un repaso.

<sup><sub><sup><sub>
o tal vez explora la historia para una pista

</sup></sub></sup></sub>

### Vuelve a tu misión

Bueno, hemos visto una gran cantidad de teoría y cómo los activos pueden (o no) interactuar y transferirse entre Stellar clásico y Soroban. Ahora es hora de que vayas y lo hagas realidad.

Si olvidaste cuál es tu tarea, aquí está de nuevo:

* [ ] Desplegar el contrato`AllowanceContract` como la cuenta`Parent_Account`
* [ ] Invocar la función`init` del contrato`AllowanceContract`
* [ ]`importar` algunos XLM en la cuenta`Parent_Account` desde su cuenta de Stellar clásico
* [ ]`aprobar` el contrato`AllowanceContract` para hacer transferencias de proxy de la cuenta`Parent_Account` a la cuenta`Child_Account`
* [ ] Invocar la función`withdraw` del contrato`AllowanceContract` con la cuenta`Child_Account` o la cuenta`Parent_Account`
* [ ]`exportar` algunos XLM desde la cuenta`Child_Account` a su cuenta de Stellar clásico

Al realizar los pasos anteriores, querrás considerar la cantidad de XLM que estás usando en el camino. En Soroban, la mayoría de los activos se cuantifican utilizando [Stroop]s (es decir, una décima millonésima del activo). Por ejemplo, si quieres importar 1 XLM, necesitarás suministrar `10000000`, `10_000_000` o `1 * 10**7` Stroops como argumento en tu invocación.

Finalmente, dada esta flexibilidad, se debe tener un gran cuidado al llamar a
las variadas invocaciones, ya que no deseas habilitar una `retirada` que sea
mayor que el permiso disponible del contrato. Elija sabiamente sus números,
mi amigo.

> Si realmente estás confundido sobre las unidades, dígitos y números a usar, lee el archivo `src/test.rs` para obtener algo de inspiración y ver qué números usamos durante el desarrollo.

## Further Reading


- **[Assets][docs-assets]** in Stellar are an enormous part of the network
  architecture. If you're unfamiliar with how assets work with "Classic"
  Stellar, than the Developer Documentation has all the information you'll need.
  Or, if you want to earn some more sweet badges while you learn, level 1 of
  [Stellar Quest Learn][sq-learn] is exactly what you want! Lots of excellent
  knowledge about assets and payments there.
- Today's quest makes use of the **[`soroban-auth` SDK][rust-auth]** for the
  first time in any quest. It allows you and your contract to authenticate users
  in a variety of ways. If you're building something on Soroban, you'll want to
  become familiar with this SDK
- Soroban doesn't know a _whole lot_ about the state of the Stellar network at
  execution time. But, it does know a few things, and those are presented to it
  as a `Ledger` data structure. There is pretty significant stuff to know, so
  here's the relevant **[documentation page][sdk-ledger]** all about it!

## Lectura Adicional

* **[Propuesta Avanzada de Núcleo 0046-06][cap-46-6]** contiene más información
  de la que probablemente quieras sobre cómo se pretende que funcione la
  interoperabilidad de activos. Estos documentos "CAP" son excelentes recursos
  para descubrir no solo*cómo* funciona algo en Stellar, sino también*por qué*
  está diseñado de esa manera. Echá un vistazo en algún momento.
* El artículo**[Contrato de Tokens Integrado][token-contract]** en la
  documentación de Soroban es un recurso probablemente menos intimidante. Tiene
  mucho más material bueno de lo que incluso podríamos mencionar aquí. Este es
  definitivamente uno para leer de principio a fin.
* Los contratos de ejemplo de**[Timelock]** ,**[Single Offer Sale][single-offer]**
  y**[Liquidity Pool][liquidity-pool]** son un gran lugar para aprender más
  sobre cómo los activos en Soroban pueden interactuar entre sí y cómo podrían
  ser interactuados. Estos son excelentes ejemplos para un uso real de los
  conceptos que hemos discutido hoy.
* **[Activos][docs-assets]** en Stellar son una parte enorme de la arquitectura de la red. Si no está familiarizado con cómo funcionan los activos con "Classic" Stellar, la documentación para desarrolladores tiene toda la información que necesitará. O, si quieres ganar algunos escudos más mientras aprendes, el nivel 1 de [Stellar Quest Learn][sq-learn] es exactamente lo que quieres! Hay mucho conocimiento excelente sobre activos y pagos allí.
* La búsqueda de hoy hace uso del**[SDK `soroban-auth`][rust-auth]** por primera vez en cualquier búsqueda. Le permite a usted y a su contrato autenticar usuarios de diversas maneras. Si está construyendo algo en Soroban, querrá familiarizarse con este SDK.
* Soroban no sabe mucho sobre el estado de la red Stellar en el momento de la ejecución. Pero, sabe algunas cosas, y esas se le presentan como una estructura de datos`Ledger`. Hay cosas significativas para saber, ¡así que aquí está la relevante**[página de documentación][sdk-ledger]** todo sobre eso!

## Still Stuck?

Si aún te encuentras atascado, revisa esta sección en nuestro archivo README principal
([aquí](https://chat.openai.com/README.md#feeling-lost)). Tiene algunas sugerencias de dónde ir desde aquí.
