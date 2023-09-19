ENTRY(start)

SECTIONS {
	/* Skip the first megabyte */
	. = 1M;

    .boot : { *(.multiboot_header) }

    .data : { *(.data) }

    .text : { *(.text) }

}
