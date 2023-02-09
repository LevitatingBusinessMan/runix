/*
Use with 
ld -n -o kernel.bin -T link.ld multiboot_header.o boot.o

-n disables paging

*/
ENTRY(start)

SECTIONS {
	/* Skip the first megabyte */
	. = 1M;

    .boot : { *(.multiboot_header) }

	.data : { *(.data) }

    .text : { *(.text) }

}
