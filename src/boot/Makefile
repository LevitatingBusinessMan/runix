iso: link
	mkdir -p isofiles/boot/grub
	cp kernel.bin isofiles/boot/kernel.bin
	cp grub.cfg isofiles/boot/grub/grub.cfg
	grub-mkrescue -o runix.iso isofiles

build: multiboot_header.asm boot.asm
	nasm -felf64 multiboot_header.asm
	nasm -felf64 boot.asm

link: build
	ld -n -o kernel.bin -T link.ld multiboot_header.o boot.o

clean:
	rm -rf *.o *.bin runix.iso isofiles

run:
	qemu-system-x86_64 -cdrom runix.iso

debug:
	qemu-system-x86_64 -cdrom runix.iso -s -S
