global _start

_start:
    ; Compute 0x54 - 0x6C and store the result to _obuf
    mov eax, 0x54
    sub eax, 0x6C
    bswap eax
    mov ecx, _obuf
    mov [ecx], eax

    ; Write to stdout
    mov eax, 4
    mov ebx, 1
    mov edx, 4
    int 0x80

    ; Exit
    mov eax, 1
    xor ebx, ebx
    int 0x80

_obuf:
    db 0, 0, 0, 0
