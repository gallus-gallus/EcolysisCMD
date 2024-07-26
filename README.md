[![Rust](https://github.com/gallus-gallus/EcolysisCMD/actions/workflows/rust.yml/badge.svg)](https://github.com/gallus-gallus/EcolysisCMD/actions/workflows/rust.yml)
# EcolysisCMD
EcolysisCMD: command line tools for simulation and analysis of ecological systems, written in Rust.

**Status:** This project is in its earliest stages. Current short-term goals are:
- [ ] Provide low-compute-cost Population Viability Analysis using population-level simulation.
- [ ] Provide forward simulation of population dynamics and genetic diversity using individual-level simulation, targeting predictive modeling of organisms for reintroduction programs.

**Ultimate Goal:** Provide the backend for an easy-to-use library of tools for ecologists and managers, implemented in a user-friendly UI (separate repo), specifically emphasizing accessibility for nonprofit and community conservation applications.

**Open Source!** Feel free to contribute, provide bug reports, etc.

# Installation Instructions
Coming with `Version 0.0.1`.

# Build Instructions
Check if Rust is installed by running `rustc --version` in your Command Line Interface. If you do not see a version number, [install Rust](https://www.rust-lang.org/tools/install) onto your system.
### Git Clone
Check if Git is installed by running `git --version` in your Command Line Interface. If you do not see a version number, [install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) onto your system.

Use your Command Line Interface to navigate to the file location where you want to save the project files (`cd ~/user/filepath/` in Bash/Zsh on a Unix-based system).

Run `git clone https://github.com/gallus-gallus/EcolysisCMD.git` in your Command Line Interface. The project files are now saved on your computer.

You can use `cargo run` in your Command Line Interface to run the program. The `cargo build --release` command will create an optimized binary for your system, found under `~/user/yourpath/ecolysis_rs/src/target/release/ecolysis_cmd.[binary_extnsion_for_your_system]`.
### Download Files
Download the project files by clicking on the greed "code" button at the top of this page. Select "Download ZIP" at the bottom of the dropdown. Unzip the downloaded file and save it to a location of your choice.

Use your Command Line Interface to navigate to the file location where you saved the project folder (`cd ~/user/filepath/` in Bash/Zsh on a Unix-based system).

You can now use `cargo run` in your Command Line Interface to run the program. The `cargo build --release` command will create an optimized binary for your system, found under `~/user/yourpath/ecolysis_rs/src/target/release/ecolysis_cmd.[binary_extnsion_for_your_system]`.

# Other Software
Need something else? Check out our [Other Software](https://github.com/gallus-gallus/EcolysisCMD/wiki/Similar-Software) wiki page for a list of related software packages!
