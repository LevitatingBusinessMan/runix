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

    call .confirm_multiboot
    call .check_cpuid
    call .check_longmode
    ; A20 should probably be enabled here?

    ; Setup paging
    call .setup_tables
    call .enable_paging

    ; Load GDT
    lgdt [gdt.pointer]

    jmp 8:long_mode

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

; I use a single PDP with 64 slots filled pointing to huge 2MiB pages
; to achieve 128MiB memory.
.setup_tables:
    mov eax, PDPT
    or eax, 0b11        ; Set present and writable
    mov [PML4T], eax    ; Save only PDPT in PM4LT

    mov eax, PDP
    or eax, 0b11        ; Set present and writable
    mov [PDPT], eax     ; Save only PDP in PDPT

    mov ecx, 64
    mov ebx, 0
    mov edx, 0

    ; EBX is location of page
    ; ECX is slots to fill
    ; EDX is current offset from PDP

.setup_table_pdp:
    mov eax, ebx            ; Copy PT address to PDP slot
    or eax, 0b10000011      ; Set flags (huge + writabe + presnet)
    mov [PDP + edx], eax    ; Save page address to slot
    add edx, 8              ; Next slot
    add ebx, 0x200000
    loop .setup_table_pdp

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
    extern _start
    call _start
    
; Will be initialized to 0
section .bss
align 4096 ; align to a page size
PML4T:
    resb 4096
PDPT:
    resb 4096
PDP:
    resb 4096

stack_bottom:
    ; Reserve 64 bytes for the stack
    resb 64
stack_top:
