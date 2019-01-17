# 8-bit-risc-machine
A small, 8-bit VM running a semi-RISC instruction set. Programmed in Rust/Cargo. Created for a DofE project!
## The machine
The machine is like any normal 8-bit computer, except that the fundamental flaw with 8-bit machines, namely their lack of memory, is solved not with 16-bit memory addresses, but instead with 'ports'. Each 'port' has 256 bytes of memory, and the machine has 8 ports, and has instructions for each one, like save/load. The idea is that each port has different processes attached, for example, in this VM port `100` is for I/O. Commands are entered into these ports exactly how memory is saved, removing the need for interrupt sequences.
## How to use
Currently supported commands are:
```
run - runs a .red file
```
For an example try:
```
cargo run run example.red
```
## FAQ
### WHY
Because I couldn't be bothered to learn assembly, so I decided to create my own.
