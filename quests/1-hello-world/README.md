# Quest 1 - Hello World <!-- omit in toc -->

## Sumario

Sé que estás apurado. ¡Hay Lumens en juego! Aquí te dejo la versión rápida y sucia de lo que necesitas hacer para conseguir la medalla SQ.

En esta misión, usarás tu **Cuenta Quest** para desplegar e invocar el contrato " 1 Hello World" en la Futurenet de Stellar.

## Tabla de Contenidos <!-- omit in toc -->

- [Sumario](#Sumario)
- [Cómo jugar](#como-jugar)
  - [Autenticarse con Discord](#authenticate-with-discord)
  - [Pull Nuevas Quests](#pull-in-new-quests)
  - [Retrieve your Quest Account](#retrieve-your-quest-account)
  - [Fund that Account](#fund-that-account)
  - [Quest Your Heart Out](#quest-your-heart-out)
  - [Check your Quest Answer](#check-your-quest-answer)
  - [Claim your Badge](#claim-your-badge)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Build the Contract](#build-the-contract)
  - [Run a Test](#run-a-test)
  - [Deploy to Futurenet](#deploy-to-futurenet)
  - [Invoke it on Futurenet](#invoke-it-on-futurenet)
- [Finish the Quest](#finish-the-quest)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

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

### Retrieve your Quest Account

Before you can play, you must retrieve the `Quest Keypair` for the quest you
want to play. You get that information by running the following:

```bash
sq play -i 1 # use whichever quest number you are trying to play
```

Save this information, because (trust me) you'll need it later!

### Fund that Account

We even put together a handy way for you to get your Futurenet Lumens from
Futurenet Friendbot (you might know his cousin...). `sq` can help you with that
like so:

```bash
sq fund --key GDGYB5FZUKAVPYGCLJTCYYOJPEHHVOCZS7I6SBWF233OQSIROZ7JXLGO
```

### Quest Your Heart Out

Now you're ready to move on to the actual quest part of this! Please skip ahead
to [this section](#the-task-at-hand) to begin the fun part!

When you think you've finished all that is required, come back here and check
your work!

### Check your Quest Answer

You've done the hard work, and you're ready to see if it has paid off! Run the
following command to verify your work:

```bash
sq check -i 1 # use whichever quest number you are trying to verify
```

If you still haven't completed KYC and/or tax submission, you'll be reminded one
more time that you can't claim XLM rewards without them. If you still have more
work to do, you'll be given a clue as to what might be missing from your Quest
Account.

### Claim your Badge

If your check was successful, the `sq` CLI will let you know with a celebratory
emoji! You'll then be able to choose how you want to claim your prize: sign a
transaction using **Albedo**, or sign the **Raw XDR**.

If you select "Albedo," a window will open asking you to sign the transaction,
and thus allowing you to claim your badge and any prize XLM you may have earned.

If you choose "Raw XDR," the transaction will be output to the terminal window,
and you will need to sign it using Stellar Laboratory (or your preferred
method). Then you must submit the signed transaction XDR using the `sq CLI`.
That will look something like this:

```bash
sq submit --xdr AAAAAgAAAADQTypLJCls2UK4wzQpHyTOdkEBKb78PvEFf7/UqD0P4gAPQkAACutwAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAACgAAAAdTdGVsbGFyAAAAAAEAAAAMUXVlc3QgUnVsZXMhAAAAAAAAAAGoPQ/iAAAAQGsvY+U2M6kGvsbJ+82A8lAQbZG/upocKynAFvADJETNUSbzMtG51KyQdetujsswz9rDDnjPosfZDVsq3ibUcAg=
```

Then you can pat yourself on the back and bask in the glow of your amazing new
Stellar Quest badge!

## The Task at Hand

This first quest is a pretty simple one, and it comes almost directly from the
Soroban examples, too! All we're doing here is getting our feet wet, so you
won't need to worry about *writing* any Rust contracts, or anything like that.
Here's what you need to know to complete this quest.

### Explore the Contract Code

If you open up the [`lib.rs` file](src/lib.rs), you'll be able to see some
helpful comments that briefly describe what each portion of the code is doing.

*This contract accepts an argument and responds with a greeting containing that
argument that was supplied.*

A more in-depth dissection of this example smart contract can be found in our
[Pioneer Quest][pq-lib].

### Build the Contract

We can now move on to actually *build* our contract! I know you didn't think you
would make it this far. Give yourself a pat on the back!

*The build process compiles our Rust code that is purpose-built for the
WebAssembly environment that Soroban will provide for it.*

If you need some instructions to help you along with this step, you can check
out the [build tutorial][docs-build] in the Soroban documentation.

Here's the short story version of what you'll need to build the contract. Run
these commands from within your Futurenet terminal:

```bash
# change into the quest directory
cd quests/1-hello-world
# build the contract
cargo build \
    --target wasm32-unknown-unknown \
    --release
```

> *Note*: We used to ship a `Makefile` with this repository which could run this
> build command for each quest. This introduced certain bugs with unfinished
> quests, and presented users with errors related to irrelevant quests. We no
> longer ship that `Makefile`, and encourage users to manually build each
> contract.

### Run a Test

You can also look at the [`test.rs` file](src/test.rs), and see it also has some
helpful comments, too.

*This test invokes the contract's `hello()` function, and checks to make sure
the contract gives the expected response.*

Once you've got an understanding of what's happening in the test scenario, go
ahead and run the test, ensuring that our contract is behaving properly.

Just like before file, the [Pioneer Quest][pq-test] contains a much more
elaborate explanation of this file, and what is happening along each step of the
way

To actually run the test in your quest Gitpod workspace, run these commands from
within your Futurenet terminal:

```bash
# change into the quest directory (if you're not there already)
cd quests/1-hello-world
# run the tests in that directory
cargo test
```

> *Note*: We used to ship a `Makefile` with this repository which could run this
> test command for each quest. This introduced certain bugs with unfinished
> quests, and presented users with errors related to irrelevant quests. We no
> longer ship that `Makefile`, and encourage users to manually test each
> contract.

### Deploy to Futurenet

The Stellar Futurenet is a safe playground where your contract code can live and
work while it is still in development and (potentially) unstable.

*Deploying the contract will upload the build binary file to the Futurenet,
making it readily available for invocation and use in the network's Soroban
environment.*

You can find some guidance on deploying your contract to the Futurenet in the
[deploy tutorial][docs-deploy] from the Soroban documentation.

### Invoke it on Futurenet

Now comes the exciting part! Your job is to **invoke** the smart contract that
you just uploaded to the Futurenet. Doesn't it just make you want to shout with
excitement!?

*Invoking your contract will use the Stellar network to call up the contract you
just uploaded and execute the `hello()` function.*

The [deploy tutorial][docs-deploy] can also give you some guidance on the finer
points of invoking a Futurenet contract, if you need.

## Finish the Quest

If you've made it this far, and you haven't blown up anything, you're probably
on the right track! Now's a great time to use the `sq` CLI to check your work,
and try to claim your prize!

You can find those [instructions here](#check-your-quest-answer).

## Further Reading

Now that you've completed your quest, you might have your interest piqued, and
be curious about all sorts of things. Here's a collection of some related
resources that might be of interest to you:

- The official [Soroban Site][soroban] contains announcements, even more
  resources, exciting learning opportunities, and more!
- The [Soroban CAP][cap] (and the related ones) outline the design choices and
  implementations of the Soroban smart contract platform. (Warning: these
  documents can be quite technical.)
- The Soroban documentation contains a number of [example contracts][examples]
  that can help you wrap your mind around even more of the concepts and ideas
  Soroban is built on.
- You can learn more about the [contract lifecycle][contract-lifecycle] in the
  "Learn" section of the Soroban documentation. There are tons of great articles
  here, and you should read through as many of them as you can!

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

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
