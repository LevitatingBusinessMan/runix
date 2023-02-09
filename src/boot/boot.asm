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

section .bss
align 4096 ; align to a page size
stack_bottom:
    ; Reserve 64 bytes for the stack
    resb 64
stack_top:
