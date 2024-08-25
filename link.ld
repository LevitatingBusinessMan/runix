ENTRY(start)

/* also see ld -verbose */

SECTIONS {
	/* Skip the first megabyte */
	. = 1M;

    .boot : { *(.multiboot_header) }

    .rodata : { *(.rodata .rodata.* .gnu.linkonce.r.*) }

    .data : { *(.data .data.*) }

    .bss : { *(.bss .bss.*) }

    .text : { *(.text .text.*) }

}
