# PokerMoons Chip Display in W.A.S.M. via RUST

The poker moons chip display takes in a dollar amount, and calculates the individual stacks of poker chips to visualize that amount given the poker chip denominations.

> <img src="./assets/example.png" alt="example chip display" style="width:250px;border-radius:5px;"/>

For curiosity sake I've compared the performance of rendering our poker chips in `Angular` vs W.A.S.M `Rust`.

## Results:

These values were tested on the default `Production` builds of each approach. For a small stack size, it's
basically impossible to tell the difference visually. But with the 5 stacks test, it is extremely noticeable.
Each environment is essentially running equivelent code, but one is in `Rust` for WASM, and the other is in `Typescript` for angular.

### Render of `1 x $4,999,999` chip stack

> <img src="./assets/1-stack.png" alt="single chip stack" style="width:250px;border-radius:5px;"/>

`Angular`

- ~25ms per render

`WASM/Rust`

- ~1ms per render

### Render of `5 x $4,999,999` chip stacks

> <img src="./assets/5-stack.png" alt="5 chip stacks" style="width:250px;border-radius:5px;"/>

`Angular`

- ~130ms per render

  > <img src="./assets/angular-5x.png" alt="Profiler of angular 5x" style="width:auto;border-radius:5px;"/>

`WASM/Rust`

- ~2.5ms per render

  > <img src="./assets/wasm-5x.png" alt="Profiler of wasm 5x" style="width:auto;border-radius:5px;"/>
