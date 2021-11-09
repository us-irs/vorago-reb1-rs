# Rust BSP for the Vorago REB1 development board

This is the Rust **B**oard **S**upport **P**ackage crate for the Vorago REB1 development board.
Its aim is to provide drivers for the board features of the REB1 board

The BSP builds on top of the [HAL crate for VA108xx devices](https://github.com/robamu-org/va108xx-hal-rs).

## Building

Building an application requires the `thumbv6m-none-eabi` cross-compiler toolchain.
If you have not installed it yet, you can do so with

```sh
rustup target add thumbv6m-none-eabi
```

This repository provides some example applications to show how the BSP is used. For example
you can build the blinky example with

```sh
cargo build --example blinky-leds
```

## Flashing from the command line

A `jlink.gdb` file is provided to allow flashing of the board from the command line.


1. Ensure that you have a suitable GDB application like `arm-none-eabi-gdb` or `gdb-multiarch`
   installed first. On Windows, you can use [xPacks](https://xpack.github.io/arm-none-eabi-gcc/).
   On Linux, you can install `gdb-multiarch` from the package manager.

2. Install the [JLink Tools](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack).

3. Start the JLink GDB server with the GUI or from the command line. The device should be recognized
   automatically

4. Make sure to select an appropriate runner in the `.cargo/config.toml` file depending on which
   GDB application you are using

5. Use

   ```sh
   cargo run --example blinky-leds
   ```

   to flash the board. The debugger should stop at the start of the main.

## Debugging with VS Code

The REB1 board features an on-board JTAG, so all that is required to flash the board is a
Micro-USB cable and an 
You can debug applications on the REB1 board with a graphical user interface using VS Code with
the [`Cortex-Debug` plugin](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug).

Some sample configuration files for VS code were provided as well. You can simply use `Run and Debug`
to automatically rebuild and flash your application.

## Flashing the non-volatile memory

Coming Soon
