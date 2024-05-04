# SoKSI - Self-Organized Kinetic Stochastic Interaction
## Research in modeling Power Laws

### Overview
A exploratory model investigating the critical dynamics prevalent in complex systems, focusing on the depiction of the cascading events that lead to critical transitions.

This model has been developed to investigate the critical dynamics prevalent in complex systems. This model, named the Self-Organized Kinetic Stochastic Interaction model, extends traditional cellular automata by incorporating non-deterministic rule sets for state transitions, which are governed by interactions among elements—specifically grains—during avalanche events. The model integrates stochasticity through a power-law-distributed function, enhancing the system's heterogeneity, And simulates the kinetic energy and the consequent momentum transfer in collisions, a method not typically observed in cellular automata frameworks.

## Setup / Use
###
Clone / Download zip

### Pre-requisites 
Install Rust [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Configure SoKSI parameters
Open the [src/util/constants.rs file](src/util/constants.rs) and configure the parameters

### Test Run 
```
cargo run
```

## Building

If desired, you can build Rust-template yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```

## Contribution

Found a problem or have a suggestion? Feel free to open an issue.

## License

Rust-template itself is licensed under the [BSD 3-Clause License](LICENSE) and includes this as the default project license.