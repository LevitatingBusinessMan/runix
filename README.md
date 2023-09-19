# Build rust portion
```SH
cargo build --target x86-runix.json -Z build-std=core
```

# Debug kernel
```SH
$ qemu-system-x86_64 -cdrom runix.iso -no-shutdown -no-reboot -d int -s -S
$ gdb src/boot/kernel.bin
(gdb) target remote localhost:1234
(gdb) c
```
