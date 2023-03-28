global start

section .data
    ;ERR db 0x4f524f45, 0x4f3a4f52, 0x4f204f20

section .text
bits 32
OK equ 0xF04BF04F ; black on white
VGA equ 0xb8000
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

    call .setup_tables
    call .enable_paging

    ; print `OK` to screen
    mov dword [VGA], OK

    hlt

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

; This will print ERR: to the screen
; followed by the error code.
; Parameters:
; error code in ax
.error:
    ; VGA reads every two words from right to left
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

; Using huge pages is too easy of course
; We use 1/4 of a PDPT (128 PDP entries), so we have 256MiB of memory
.setup_tables:
    mov eax, PML4T      ; Get address of PML4T (only 52 bits used as address)
    add eax, 0x1000     ; PDPT is 4096 bytes further
    or eax, 0b11        ; Set the first 2 bits, present and writable
    mov [PML4T], eax    ; Set the first entry of PML4T to our only PDPT

    mov ecx, 128        ; PDP's to make
    mov edx, 512        ; PT's to make per PDP

    ; EAX is currently the address of the PDPT
    
    mov ebx, eax        ; copy EAX
    add ebx, 0x1000     ; Adress of the first PDP
    or ebx, 0b11        ; Set first 2 bits

    ; EAX tracks the current slot in the PDPT
    ; EBX tracks the current slot in the PDP

.setup_table_pdp:
    mov edi, ebx            ; Copy address of this PDP
    mov dword [eax], ebx    ; Set address of this PDP IN PDPT
    add ebx, 0x1200         ; Add size of a PDP (512 PT's and itself) to get address of next PDP
    add eax, 8              ; Adress of next slot in PDPT
    mov edx, 512            ; PT's to make

.setup_table_pt:
    mov dword [ebx], edi    ; Set address of this PT in PDP
    add edi, 0x1000         ; Address of next PT
    add ebx, 8              ; Adress of next slot in PDP
    dec edx
    jnz .setup_table_pt
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

section .bss
align 4096 ; align to a page size
PML4T:
    ; 1 PML4T
    ; 1 PDPT
    ; 128 PDP
    ; 512 PT
    resb 1 * 1 * 128 * 512 * 8

stack_bottom:
    ; Reserve 64 bytes for the stack
    resb 64
stack_top:
