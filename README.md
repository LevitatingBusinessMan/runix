Runix is a WIP unikernel. The name "runix" stands for: Runix Has Nothing To Do With Unix.

# Welcome page
![](screenshots/welcome_to_runix2.png)

# Red screen of death
![](screenshots/red_screen_of_death.png)

# Building and running
```
$ make
$ make run
```

# Debug kernel
```SH
$ qemu-system-x86_64 -cdrom runix.iso -no-shutdown -no-reboot -d int -s -S
$ gdb src/boot/kernel.bin
(gdb) target remote localhost:1234
(gdb) c
```
