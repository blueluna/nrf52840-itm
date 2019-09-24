# ITM experiments on the nRF52840-DK

## Starting the debugger

Get JLinkGDBServer from Segger.

Use the `jlinkgdb` shell script to start the GDB server.

```
$ ./jlinkgdb
```

In another terminal, connect to the SWO port using telnet.

```
$ telnet localhost 2332
```

In yet another terminal. Run the program in the debugger.

```
$ cargo run
...
SWO enabled successfully.
Resetting target
Loading section .vector_table, size 0x100 lma 0x0
Loading section .text, size 0x2f0a lma 0x100
Loading section .rodata, size 0x8e8 lma 0x3020
Start address 0x2d80, load size 14578
Transfer rate: 7118 KB/sec, 4859 bytes/write.
(gdb)
```

press `c` to continue

Expected output on telnet is,

```
Initialize
...
panicked at 'end', src/main.rs:28:9
```
