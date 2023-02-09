global start

section .data
    ;ERR db 0x4f524f45, 0x4f3a4f52, 0x4f204f20

section .text
bits 32
OK equ 0xF04BF04F
VGA equ 0xb8000
start:
    ; OK black on white

    ; print `OK` to screen
    mov dword [VGA], OK
    hlt

; This will print ERR: to the screen
; followed by the error code.
; Parameters:
; error code in al
error:
    mov dword [VGA],  0xF452F445        ; ER
    mov dword [VGA + 4], 0xF43AF452     ; R:
    mov dword [VGA + 8], 0xF420F420     ; two spaces
    mov byte  [VGA + 10], al            ; replace second space with error
    hlt
