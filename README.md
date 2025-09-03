# SeccampConnectPre

Security Camp Connect Pre 2025

This repository is based on the [rmk example](https://github.com/HaoboGu/rmk/tree/rmk-v0.7.8/examples/use_rust/esp32c6_ble).

## Build

To run this example, use the following command:

```
$ cargo run --release
    Compiling ...
    ...
    ...
    Finished `release` profile [optimized + debuginfo] target(s) in 11.70s
     Running `espflash flash --monitor --port /dev/cu.usbmodem211401 target/riscv32imac-unknown-none-elf/release/rmk-esp32c6`
[2025-04-10T10:01:23Z INFO ] Serial port: '/dev/cu.usbmodem211401'
[2025-04-10T10:01:23Z INFO ] Connecting...
[2025-04-10T10:01:23Z INFO ] Using flash stub
Chip type:         esp32c6 (revision v0.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       40:4c:ca:5b:c7:dc
App/part. size:    768,944/4,128,768 bytes, 18.62%
[2025-04-10T10:01:23Z INFO ] Segment at address '0x0' has not changed, skipping write
[2025-04-10T10:01:23Z INFO ] Segment at address '0x8000' has not changed, skipping write
[00:00:06] [========================================]     411/411     0x10000
```
