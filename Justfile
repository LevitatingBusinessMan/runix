default: esp

kernel:
  cargo build 

esp: kernel
  rm -rf esp
  mkdir esp
  mkdir -p target/esp/EFI/BOOT
  mkdir -p target/esp/boot
  cp Limine/BOOTX64.EFI target/esp/EFI/BOOT
  cp target/x86_64-unknown-none/debug/runix target/esp/boot/runix.elf
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

go: esp run

# QEMU
# disk images may be run with -drive format=raw,file=runix.img

run:
  qemu-system-x86_64 \
    -bios /usr/share/qemu/ovmf-x86_64.bin \
    -drive format=raw,file=fat:rw:target/esp \
    -no-shutdown -no-reboot \
    -debugcon stdio \
    -m 256M

monitor:
    qemu-system-x86_64 \
    -bios /usr/share/qemu/ovmf-x86_64.bin \
    -drive format=raw,file=fat:rw:target/esp \
    -no-shutdown -no-reboot \
    -monitor stdio \
    -m 256M

debug:
  qemu-system-x86_64 \
    -bios /usr/share/qemu/ovmf-x86_64.bin \
    -drive format=raw,file=fat:rw:target/esp \
    -no-shutdown -no-reboot \
    -m 256M \
    -debugcon stdio \
    -s -S &
  gdb -x gdb.commands target/esp/boot/runix.elf
