ENTRY _start

_lhs:
    DL 0x6C

_rhs:
    DL 0x54

_buffer:
    DS 255

_start:
    ; Compute _lhs - _rhs and write the result to stdout in base 16 (EBX)
    mov eax, [_lhs]
    mov ebx, [_rhs]
    sub eax, ebx
    mov ebx, 16
    mov ecx, _buffer
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

