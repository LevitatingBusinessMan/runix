default: image

kernel:
  cargo build

boot:
	mkdir -p target
	nasm -felf64 src/boot/multiboot_header.asm -o target/multiboot_header.o
	nasm -felf64 src/boot/boot.asm -o target/boot.o

elf: boot kernel
  ld -n -o target/runix.elf -T link.ld target/multiboot_header.o target/boot.o target/x86_64-unknown-none/debug/librunix.a

esp: elf
  rm -rf esp
  mkdir esp
  mkdir -p target/esp/EFI/BOOT
  mkdir -p target/esp/boot
  cp Limine/BOOTX64.EFI target/esp/EFI/BOOT
  cp target/runix.elf target/esp/boot
  cp limine.conf target/esp/boot

image: esp
  rm -f runix.img
  PATH=/sbin:$PATH systemd-repart --empty=create --size=auto --definitions=repart.d --copy-source=. --dry-run=no runix.img

limine-bin:
  cd Limine && make

clean:
  rm -f runix.mg
  rm -rf target
  rm -rf esp

run:
  qemu-system-x86_64 \
    -bios /usr/share/qemu/ovmf-x86_64.bin \
    -drive format=raw,file=runix.img \
    -no-shutdown -no-reboot \
    -m 512M

debug:
  qemu-system-x86_64 \
    -bios /usr/share/qemu/ovmf-x86_64.bin \
    -drive format=raw,file=runix.img \
    -no-shutdown -no-reboot \
    -m 512M
