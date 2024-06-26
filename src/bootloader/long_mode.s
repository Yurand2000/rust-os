global long_mode_start
extern kernel_start

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; print `OKAY` to screen
    mov rax, 0x1f591f411f4b1f4f
    mov qword [0xb8000], rax
    call kernel_start
    hlt