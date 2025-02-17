entry _start

_lhs:
    db 0x6C, 0x00, 0x00, 0x00

_start:
    ; Compute _lhs - 0x54 and write the result to stdout in base (EBX)
    mov eax, [_lhs]
    sub eax, 0x54
    mov ebx, 10
    mov ecx, 0x09000000
    xor edi, edi

_loop:
    xor edx, edx
    div ebx
    cmp dl, 9
    jle _skip_a
    add dl, 0x07
_skip_a:
    add dl, 0x30
    dec ecx
    inc edi
    mov [ecx], dl
    cmp eax, 0
    jnz _loop

    ; Write to stdout
    mov eax, 4
    mov ebx, 1
    mov edx, edi
    int 0x80

    ; Exit
    mov eax, 1
    xor ebx, ebx
    int 0x80

