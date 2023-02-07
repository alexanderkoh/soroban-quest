# Quest 5 - Tipos personalizados <!-- omit in toc -->

## TL;DR

¡Vale! Supongo que *tal vez* puedas empezar con solo un corto estímulo. ¡De acuerdo! ¡Sé mi invitado! Pero no te olvides: ¡El conocimiento abunda más abajo en el README!

**ADEMÁS** : ¡Hoy vamos a cambiar las cosas! ¡Te has vuelto bastante bueno en los pasos de implementación e invocación del proceso de Soroban (y, sí, tendrás que hacerlo hoy también)! ¿Pero cómo están tus habilidades en Rust?? ¡Eso es correcto! ¡Hoy escribirás algo de Rust real en tu contrato!

**Para la quest de hoy, debes crear un conjunto de tipos personalizados en tu contrato que cumplan con las especificaciones listadas a continuación. También debes implementar ese contrato y luego realizar las invocaciones necesarias para cada uno de los tipos personalizados. Finalmente, debes invocar la función `verify` de nuestro contrato de verificación `422d1c0091a0528e3630fb5301389da530c40be4ce2eb8fcadf426a3d53627d3` para que podamos verificar tus definiciones de tipos personalizados.**

## Tabla de contenido <!-- omit in toc -->

* [TL;DR](https://chat.openai.com/chat#tldr)
* [Cómo jugar](https://chat.openai.com/chat#c%C3%B3mo-jugar)
* [La tarea a mano](https://chat.openai.com/chat#la-tarea-a-mano)
  * [Tipos personalizados en Rust](https://chat.openai.com/chat#tipos-personalizados-en-rust)
    * [`enum`s de Rust](https://chat.openai.com/chat#enums-de-rust)
    * [`struct`s de Rust](https://chat.openai.com/chat#structs-de-rust)
  * [Tipos personalizados en Soroban](https://chat.openai.com/chat#tipos-personalizados-en-soroban)
  * [Crea tus tipos personalizados](https://chat.openai.com/chat#crea-tus-tipos-personalizados)
    * [Rectángulo](https://chat.openai.com/chat#rect%C3%A1ngulo)
    * [Animal](https://chat.openai.com/chat#animal)
    * [Usuario](https://chat.openai.com/chat#usuario)
    * [RGB](https://chat.openai.com/chat#rgb)
    * [Color](https://chat.openai.com/chat#color)
    * [Participante](https://chat.openai.com/chat#participante)
    * [RoyalCard](https://chat.openai.com/chat#royalcard)
  * [Invoca tu contrato](https://chat.openai.com/chat#invoca-tu-contrato)
  * [Deja que verifiquemos tu trabajo](https://chat.openai.com/chat#deja-que-verifiquemos-tu-trabajo)
* [Lectura adicional](https://chat.openai.com/chat#lectura-adicional)
* [Todavía atascado?](https://chat.openai.com/chat#todav%C3%ADa-atascado)

## Cómo jugar

Si te perdiste de nuestras misiones anteriores o solo necesitas un repaso, tenemos instrucciones (bastante extensas) para las *mecánicas* de completar estas misiones (generar parejas de claves, revisar tu trabajo, etc.).

Toda esa información [se puede encontrar aquí][how-to-play] si necesitas usar esas instrucciones nuevamente.

## La tarea a mano

Este es un momento importante! Te sientes preparado. Te sientes listo. Francamente, *estás* listo! Tienes el valor y la tenacidad para abordar esta misión!

Pero, de nuevo, por favor: **Lee el código!!** Hay cosas importantes que necesitas
saber dentro de él. (Además, trabajamos muy duro en él, y deberías usarlo al
máximo).

*Bonificación* : Ya nos has escuchado decir que debes "leer `lib.rs`" como un centenar de veces. Pero hoy hay un nuevo y elegante archivo `types.rs` que deberías echar
un vistazo.

### Tipos Personalizados en Rust

Un tipo personalizado en el dialecto Rust de Soroban se declara usando el macro de atributo `contracttype` en una definición de `struct` o `enum`. Pero, antes de sumergirnos en la información específica de Soroban, detengámonos en cómo este concepto se desarrolla en un entorno estándar de Rust. No te preocupes, esto será rápido.

#### Enums de Rust

En Rust, podemos usar una `enum` (abreviatura de `enumeración`) para definir un tipo al listar (o "enumerar") todas sus variantes posibles. Podrías pensar en una `enum` definida como un "menú" del que eliges un elemento. No necesariamente elegirás lo mismo cada vez, pero solo elegirás uno cuando sea el momento de hacer una selección. Por ejemplo:

```rust
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    Skittles, // an important part of a balanced diet
}
```

El *Libro de Rust* contiene [todo un capítulo sobre los `enum`][rust-enums], y tiene mucha más información valiosa que puedes aprender. ¡Asegúrate de revisarlo!

#### Rust `struct`s

En Rust, una `struct` nos permite reunir y nombrar múltiples valores que están relacionados de alguna manera. Tal `struct` será un tipo de datos personalizado que representa un grupo significativo de esos valores. En lugar de un menú del cual eliges, una `struct` es más como ese documento de "plantilla" que guardas para que puedas hacer una copia y llenar algunas cosas cuando llegue el momento de comenzar tus tareas. Por ejemplo:

```rust
struct Homework {
    class: String, // remember soroban does not include `String` - it's only here as an example
    subject: String,
    studentId: u32,
    complete: bool,
    date: String,
}
```

Entonces podríamos crear una nueva *instancia* de una tarea de `Homework` haciendo algo como esto:

```rust
let homework1 = Homework {
    class: String::from("AA429"),
    subject: String::from("Advanced Astrophotography"),
    studentId: 8675309,
    complete: false,
    date: String::from("Next Wednesday"),
};
```

El libro de *Rust* también tiene un [capítulo completo sobre `struct`s][rust-struct], y ¡tiene aún más información! ¡Echa un vistazo a este también!

### Tipos Personalizados en Soroban

Si eliges hacer clic en solo un enlace en todo este README, por favor hazlo en este: El artículo de **[Tipos Personalizados][learn-ct]** en la documentación de Soroban es realmente muy bueno. Los tipos personalizados que puedes crear en Soroban están compuestos por tipos `struct` y tipos `enum`, aunque hay algunas convenciones diferentes para definir esos tipos. Las grandes categorías de tipos personalizados que puedes crear son:

* [Struct con Campos con Nombre](https://soroban.stellar.org/docs/learn/custom-types#structs-with-named-fields)
* [Struct con Campos sin Nombre](https://soroban.stellar.org/docs/learn/custom-types#structs-with-unamed-fields)
* [Enum con Variantes de Unidad y Tupla](https://soroban.stellar.org/docs/learn/custom-types#enum-unit-and-tuple-variants)
* [Enum con Variantes Enteras](https://soroban.stellar.org/docs/learn/custom-types#enum-integer-variants)

También es importante entender que los enums solo son compatibles como tipos de contrato en Soroban cuando todas las variantes tienen un literal entero explícito, o cuando todas las variantes son variantes unitarias o de un solo campo.

En el artículo **[Tipos personalizados][learn-ct]** , aprenderás mucho sobre cómo Soroban almacenará tus tipos personalizados en el Ledger, la conversión XDR, la representación JSON y más.

Encontrarás un recurso adicional (y muy útil) en la documentación de Soroban aquí: [Enumeraciones de errores][error-enums] describe cómo podrías usar una `enum` para transmitir de manera significativa la información de error. Eso podría parecer vagamente familiar, si recuerdas haber tenido que resolver frenéticamente qué error estabas recibiendo (y por qué) durante [La Misión 2](https://chat.openai.com/2-auth-store/README.md).

### Crea tus tipos personalizados

Ok, eso fue un gran contenido educativo, ¡pero volvemos al camino! Para esta búsqueda, debes crear y luego invocar los siguientes tipos personalizados en tu contrato:

#### Rectángulo

El tipo `Rectángulo` debe ser un `struct`, con dos campos: `width` y `height` que ambos deben ser un valor `u32`.

Invoque la función `c_rect` para crear un `Rectángulo` usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_rect \
    --arg '{"object":{"map":[{"key":{"symbol":"width"},"val":{"u32":<a-u32-integer>}},{"key":{"symbol":"height"},"val":{"u32":<a-u32-integer>}}]}}'
```

#### Animal

El tipo `Animal` debe ser un `enum`, con al menos dos variaciones: `Cat` y `Dog`.

Invoca la función `c_animal` para crear un `Animal` usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_animal \
    --arg '{"object":{"vec":[{"symbol":"<a-relevant-animal-symbol>"}]}}'
```

#### User

El tipo `User` debe ser un `struct` con campos `name`, `age` y `pet`, correspondientes a valores de `Bytes`, `u32` y `Animal`, respectivamente.

Invoque la función `c_user` para crear un `User` usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_user \
    --arg '{"object":{"map":[{"key":{"symbol":"name"},"val":{"object":{"bytes":"<a-hex-encoded-string>"}}},{"key":{"symbol":"age"},"val":{"u32":<a-u32-integer>}},{"key":{"symbol":"pet"},"val":<an-animal-object>}]}}'
```

*Nota*: Necesitarás usar algo de JSON para el campo "Animal" del usuario. Incluso podrías copiar el argumento que usaste al invocar la función "c_animal".

#### RGB

El tipo `RGB` debe ser un tipo de estructura de `tupla` formada por una `tupla` de `3`valores`u32`.

Invoque la función `c_rgb` para crear un valor `RGB` usando algo así como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_rgb \
    --arg '{"object":{"vec":[{"u32":<a-u32-integer>},{"u32":<a-u32-integer>},{"u32":<a-u32-integer>}]}}'
```

#### Color

El tipo `Color` combinará el tipo personalizado `RGB` anidado dentro de un tipo de enumeración de tuplas. Construye tu tipo de estructura `RGB` como se describe arriba, luego tu tipo de enumeración `Color` debe definirse como una variante con un nombre de "RGB" e una instancia de tu tipo `RGB`.

Invocar la función `c_color` para crear un `Color` usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_color \
    --arg '{"object":{"vec":[{"symbol":"RGB"},<a-rgb-object>]}}'
```

#### Participant

El tipo `Participant` debe ser un `enum` con variantes de tuplas de un solo valor como sigue:

* Una variante "Cuenta" con un tipo`AccountId`
* Una variante "Contrato" con un tipo`BytesN<32>`

Invoca la función `c_part` para crear un participante de cuenta usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_part \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"<hex-encoded-account-id>"}}}]}}'
```

También invoca la función `c_part` para crear un participante de contrato usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_part \
    --arg '{"object":{"vec":[{"symbol":"Contract"},{"object":{"bytes":"<contract-id>"}}]}}'
```

#### RoyalCard

El tipo `RoyalCard` debe ser un `enum` que contenga tres variaciones de enteros `u32` como sigue:

* Una variante "Jack", con un valor de 11
* Una variante "Queen", con un valor de 12
* Una variante "King", con un valor de 13

Invocar la función `c_card` usando algo como:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_card \
    --arg '{"u32":<a-u32-integer>}'
```

### Invocar tu Contrato

¡Eso fue mucho trabajo, ¿verdad? Debes estar realmente orgulloso de ti mismo. Yo lo estoy. Ahora que tienes todos tus tipos personalizados escritos y desplegados (¡oh sí, no olvides desplegar tu contrato!), debes *invocar* cada una de las funciones listadas en `src/lib.rs` y pasar un `--arg` válido para tu tipo personalizado. Hay algunas sugerencias útiles sobre cómo invocar estos a lo largo de este README.

Por si perdiste la cuenta, debes invocar las siguientes funciones proporcionando un argumento del tipo personalizado que creaste:


| Función | Tipo de Argumento                                         |
| ---------- | ----------------------------------------------------------- |
| c_rect   | [Rectángulo](https://chat.openai.com/chat#rectangle)     |
| c_animal | [Animal](https://chat.openai.com/chat#animal)             |
| c_user   | [Usuario](https://chat.openai.com/chat#user)              |
| c_rgb    | [RGB](https://chat.openai.com/chat#rgb)                   |
| c_color  | [Color](https://chat.openai.com/chat#color)               |
| c_part   | [Participante](https://chat.openai.com/chat#participant)* |
| c_card   | [RoyalCard](https://chat.openai.com/chat#royalcard)       |

> * No olvides que el tipo `Participante` debe ser invocado dos veces, una vez como una `Cuenta`, y una vez como un `Contrato`.
>

### Permítenos Revisar tu Trabajo

Bien hecho, has personalizado todos los tipos, has invocado todas las cosas, y
estás listo para reclamar tu premio. Antes de continuar, solo *una* cosa más:
**Debes invocar nuestro contrato de verificación** .

Usando tu par de claves Quest, debes invocar la función `verificar` en el contrato con el ID `40d12b03a08f5dde4e0068aa752fa65eddf905e82a18f522efe350e0cd268b8a`,
suministrando tu propio ID de contrato como argumento. Revisaremos todo tu arduo trabajo, y nos aseguraremos de que hayas implementado los tipos personalizados requeridos con los campos, variantes, valores, etc. necesarios.

**Entonces** , eres libre de usar `sq check 5` para (intentar) reclamar tu premio!

### Lectura Adicional

* De nuevo, solo confía en mí y**lee esto** : el artículo [Tipos Personalizados][learn-ct]
  en la sección Aprender de la documentación de Soroban es*imprescindible* .
* Puedes buscar en las [Documentación del SDK][sdk-contracttype] para obtener más información sobre la
  macro de atributo`contracttype`.
* Hay un [contrato de ejemplo de Tipos Personalizados][example-ct] completo que puedes ver y leer
  en la documentación de Soroban. Es excelente para inspiración o ver
  cómo todas estas piezas pueden encajar juntas.
* Lee más sobre el [Dialecto de Contrato][soroban-dialect] de Rust utilizado en
  Soroban en la documentación.
* ¡Lo mencionamos varias veces la semana pasada, pero en serio! Confía en nosotros cuando te decimos
  que la [Guía de Smephite][smephite-guide] es increíblemente útil!

## ¿Todavía atascado?

Si te encuentras en una pared, y no estás seguro de cuál es tu próximo movimiento, consulta [esta sección](https://chat.openai.com/README.md#feeling-lost) en nuestro README principal. Tiene algunas sugerencias sobre adónde ir a partir de aquí

[how-to-play]: ../1-hello-world/README.md#how-to-play
[sdk-contracttype]: https://docs.rs/soroban-sdk/latest/soroban_sdk/attr.contracttype.html
[learn-ct]: https://soroban.stellar.org/docs/learn/custom-types
[example-ct]: https://soroban.stellar.org/docs/examples/custom-types
[rust-struct]: https://doc.rust-lang.org/book/ch05-00-structs.html
[rust-enums]: https://doc.rust-lang.org/book/ch06-00-enums.html
[error-enums]: https://soroban.stellar.org/docs/learn/errors#error-enums
[soroban-dialect]: https://soroban.stellar.org/docs/learn/rust-dialect
[smephite-guide]: https://gist.github.com/Smephite/09b40e842ef454effe4693e0d18246d7
