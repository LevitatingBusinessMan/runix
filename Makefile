iso: link
	mkdir -p target/isofiles/boot/grub
	cp target/kernel.elf target/isofiles/boot/kernel.elf
	cp src/boot/grub.cfg target/isofiles/boot/grub/grub.cfg
	grub-mkrescue -o runix.iso target/isofiles

kernel:
	cargo build

boot: src/boot/multiboot_header.asm src/boot/boot.asm
	mkdir -p target
	nasm -felf64 src/boot/multiboot_header.asm -o target/multiboot_header.o
	nasm -felf64 src/boot/boot.asm -o target/boot.o

link: boot kernel
	ld -n -o target/kernel.elf -T link.ld target/multiboot_header.o target/boot.o target/x86-runix/debug/librunix.a

clean:
	rm -rf *.o *.bin runix.iso isofiles

run:
	qemu-system-x86_64 -cdrom runix.iso -no-shutdown -no-reboot -d int

debug:
	qemu-system-x86_64 -cdrom runix.iso -no-shutdown -no-reboot -d int -s -S
