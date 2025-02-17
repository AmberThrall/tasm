INCLUDE "hex_dump_functions.s"
ENTRY Main

Main:
    xor edi, edi
    xor esi, esi
_loop2:
    cmp esi, 0
    jnz _get_byte
    
    push edi
    ; Print "offset: \n"
    
    mov ecx, 0x09000000
    mov dl, 0xA    ; 0xA = '\n'
    mov [0x09000000], dl 
    mov edx, 1
    call Print

    mov edx, 8
    mov eax, edi 
    call PrintHex

    mov ecx, 0x09000000
    mov dl, 0x3A   ; ':'
    mov [0x09000000], dl 
    mov dl, 0x20   ; 0x20 = ' '
    mov [0x09000001], dl 
    mov edx, 2
    call Print
    mov esi, 16

    mov ecx, 0x09000000
    mov edx, 1
    call Clear
    pop edi

_get_byte:
    push esi
    push edi
    mov ecx, 0x09000000
    call GetByte
    cmp eax, 0
    jz _exit

    ; Lookup the byte
    mov eax, [ecx]

    mov edx, 2
    call PrintHex

    mov eax, 0x20
    mov [ecx], eax      ; [ecx] <- ' '
    mov edx, 1
    call Print

    pop edi
    pop esi
    inc edi
    dec esi
    jmp _loop2 

_exit:
    mov eax, 0xA    ; 0xA = '\n'
    mov [0x09000000], eax
    call Print

    ; Exit
    mov eax, 1          ; exit
    xor ebx, ebx
    int 0x80
