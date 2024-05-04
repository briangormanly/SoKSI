# Discrete Sandpile Experiment
## Research in modeling Power Laws

### Overview
The 5-discrete-critical/ folder contains the working sandpile of interest, the other folders are all pre-experiments mostly in learning rust and playing with appropriate data-structures and ui techniques. None of the other folders demonstrate criticality or power law outcomes, but instead build the model to the critical state but exhibit no invariance of scale as future additions of grains simply roll off the pile. The model in 5-discrete-critical/ does exhibit these properties and will be the only model discussed.

## Setup / Use
###
Clone / Download zip

### Pre-requisites 
Install Rust [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Configure SoKSI parameters
Open the [util/constants.rs file](util/constants.rs) and configure the parameters

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