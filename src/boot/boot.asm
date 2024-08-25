global start

section .rodata
gdt:
    dq 0;                                   ; All GDT's start with a 0 entry
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53); 64 bit ring 0 segment entry
.pointer:
    dw $ - gdt - 1                          ; SIZE of GDT
    dd gdt                                  ; Address of GDT

section .text
bits 32
OK equ 0xF04BF04F     ; black on white OK
VGA equ 0xb8000 + 160 ; start on next line
start:

    ; At this point multiboot should've set EAX to ‘0x36d76289’;
    ; And EBX points to a multiboot information structure
    ; See 3.3 I386 machine state

    ; Set the stack pointer
    mov esp, stack_top

    ; Save multiboot information
    mov edi, ebx

    call .disable_cursor
    call .confirm_multiboot
    call .check_cpuid
    call .check_longmode
    ; A20 should probably be enabled here?

    ; Setup paging
    call .setup_tables
    call .enable_paging

    ; Load GDT
    lgdt [gdt.pointer]

    ; Far jump to segment
    jmp 8:long_mode

.disable_cursor:
    ; save eax for multiboot magic
    push eax
    mov dx, 0x3D4
	mov al, 0xA
	out dx, al

    inc dx
    mov al, 0x20 ; bit 0-4 is cursor shape, bit 5 disables it
    out dx, al
    pop eax

    ret

; Confirm if booted via multiboot
.confirm_multiboot:
    cmp eax, 0x36d76289
    jne .not_multiboot
    ret
.not_multiboot:
    mov ax, "nm"
    jne .error

; The people on IRC said I was crazy for actually implementing this
; Check for CPUID capabilities (by flipping bit 21 in EFLAGS)
.check_cpuid:
    ; Push EFLAGS and pop it into EAX and ECX
    pushfd
    pop eax
    mov ecx, eax ; Don't overwrite multiboot stuff on EBX

    ; Flip bit 21
    xor eax, 1 << 21

    ; Copy EAX back into EFLAGS and back into EAX via stack
    ; If supported the bit should be flipped
    push eax
    popfd
    pushfd
    pop eax

    ; Restore original EFLAGS
    push ecx
    popfd

    xor eax, ecx
    je .no_cpuid
    ret

.no_cpuid:
    mov ax, "nc"
    jne .error

.check_longmode:

    ; This check if failing for whatver reason:
    ; Check for extended processor info availibility
    ; mov eax, 0x8000000
    ; cpuid

    ; ; If EAX < 0x80000001 there is definitely no long mode
    ; cmp eax, 0x80000001
    ; jb .no_longmode
    
    mov eax, 0x80000001
    cpuid
    ; If bit 29 is set on EDX long mode is supported
    test edx, 1 << 29 
    jz .no_longmode
    ret

.no_longmode:
    mov ax, "nl"
    jne .error

; https://en.wikipedia.org/wiki/VGA_text_mode
; This will print ERR: to the screen
; followed by the error code.
; Parameters:
; error code in ax
.error:
    ; Remember endianness
    mov dword [VGA],     0xF452F445     ; ER
    mov dword [VGA + 4], 0xF43AF452     ; R:
    mov word  [VGA + 8], 0xF420         ; a space

    mov byte  [VGA + 10], al            ; set color
    mov byte  [VGA + 11], 0xF4          ; add second character
    mov byte  [VGA + 12], ah            ; set color
    mov byte  [VGA + 13], 0xF4          ; add first character
    hlt
; ERROR CODES:
; nm          Not Multiboot
; nc          No CPUID
; nl          No long mode


; https://en.wikipedia.org/wiki/X86-64#Virtual_address_space_details
; https://os.phil-opp.com/entering-longmode/#paging
; https://wiki.osdev.org/Setting_Up_Paging


; We have one of each table. The PT table is full (to get more control of the first 2MiB of pages)
; The PDT is filled with an aditional 7 huge pages
; In total we map 16 MiB
.setup_tables:
    mov eax, PDPT
    or eax, 0b11        ; Set present and writable
    mov [PML4T], eax    ; Save only PDPT in PM4LT

    mov eax, PDT
    or eax, 0b11        ; Set present and writable
    mov [PDPT], eax     ; Save only PDP in PDPT

    mov eax, PT
    or eax, 0b11        ; Set present and writable
    mov [PDT], eax      ; Save only PDP in PDPT

    mov ecx, 512
    mov ebx, 0
    mov edx, 0

    ; EBX is location of page
    ; ECX is slots to fill
    ; EDX is current offset from table

.setup_table_pt:
    mov eax, ebx            ; Copy page address to entry
    cmp ebx, guard
    je .guard               ; Don't set present flag on guard
    or eax, 0b11            ; Set flags (writabe + present)
.guard:
    mov [PT + edx], eax     ; Save page address to slot
    add edx, 8              ; Next slot
    add ebx, 0x1000
    loop .setup_table_pt

    mov ecx, 7              ; Huge pages to create
    mov ebx, 0x200000
    mov edx, 8

.setup_table_pdt:
    mov eax, ebx            ; Copy page address to entry
    or eax, 0b10000011      ; Set flags (huge + writabe + present)
    mov [PDT + edx], eax    ; Save page address to slot
    add edx, 8              ; Next slot
    add ebx, 0x200000
    loop .setup_table_pdt

    ret

.enable_paging:
    ; Enable PAE (bit 5 of cr4)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Set LME (Long Mode Enable) (bit 8 of EFER MSR)
    mov ecx, 0xC0000080 ; EFER MSR
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Save PML4T table to CR3
    mov eax, PML4T
    mov cr3, eax

    ; Enable PG (Paging) (bit 31 of cr0)
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

section .text
bits 64
OKAY equ 0xF059F041F04BF04F
long_mode:

    ; To avoid issues with niche instructions
    ; nullify all segment registers except cs
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Call rust
    extern runix
    call runix
    
; Will be initialized to 0
section .bss
; Align with page size
align 4096
; Stack (16 pages and a guard page)
guard:
    resb 4096
stack_bottom:
    resb 4096 * 16
stack_top:

; Tables
PML4T:
    resb 4096
PDPT:
    resb 4096
PDT:
    resb 4096
PT:
    resb 4096

; EARLY BOOT LAYOUT
; link.ld shows the ELF layout.
; The first megabyte of the ELF
; is configured to have 1MiB of null bytes.
; GRUB maps this ELF at the top of memory.
; This .bss section is created somewhere with a page table
; mapping the first 16MiB of memory.
; Also in the .bss section is 16 pages (64KiB)
; reserved for the stack.
; With a non-present guard page on top.
