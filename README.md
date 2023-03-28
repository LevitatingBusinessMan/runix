# Debug kernel
$ qemu-system-x86_64 -cdrom runix.iso -s -S
$ gdb src/boot/kernel.bin
(gdb) target remote localhost:1234
(gdb) c
