; See the mulitboot.pdf

; Offset		Type		Field Name
; 0			u32			magic
; 4			u32			architecture
; 8			u32			header_length
; 12			u32			checksum
; 16-XX		u32			tags


section .multiboot_header
header_start:
	dd 0xE85250D6				; The field ‘magic’ is the magic number identifying the header,
								; which must bethe hexadecimal value 0xE85250D6.
	
	dd 0						; The field ‘architecture’ specifies the Central Processing Unit Instruction Set
								; Architecture. Since ‘magic’ isn’t a palindrome it already specifies the
								; endian-ness ISAs differing only in endianness recieve the same ID. ‘0’ means 32-bit
								; (protected) mode of i386. ‘4’ means 32-bit MIPS.
	
	dd header_end - header_start; The field ‘header_length’ specifies the Length of multiboot header in bytes
								;including magic fields.
	
	dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))
								; The field ‘checksum’ is a 32-bit unsigned value which, when added to the other
								; magic fields (i.e. ‘magic’, ‘architecture’ and ‘header_length’), must have a
								; 32-bit unsigned sum of zero.

								; I subtract from 0x10000000 instead of 0
								; to prevent an underflow
								; see https://github.com/intermezzOS/book/issues/28

	; Multiboot tags go here

	; Denote end of tags with empty tag
	dw 0						; type
	dw 0						; flags
	dd 8						; size	
header_end:
