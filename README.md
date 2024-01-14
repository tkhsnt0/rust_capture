# rust_capture
Packet capture in rust. 

# how to use
[build]
```
cargo build
```
[execute]
```
sudo ./target/debug/capture [interface]
```
The specified interface name are, for example, lo, eth0, and so on.

[example]

Displays the L4 protocol ('UDP' or 'TCP'), source [ip:port], destination [ip:port] and payload.
```
Captured a UDP packet from 172.17.96.1|5353 to 224.0.0.251|5353

00 00 00 00 00 01 00 00 00 00 00 00 04 5F 69 70 70 04 5F 74     | .t
63 70 05 6C 6F 63 61 6C 00 00 0C 80 01                                 | ..
============================================================
```
