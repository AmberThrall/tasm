INCLUDE "hex_dump_functions.s"
ENTRY Main

EQU INBUFFER  0x09000000
EQU OUTBUFFER 0x09800000

Main:
    xor edi, edi
    xor esi, esi
_loop2:
    cmp esi, 0
    jnz _get_byte
    
    push edi
    ; Print "offset: \n"
    
    mov ecx, OUTBUFFER
    mov dl, 0xA    ; 0xA = '\n'
    mov [OUTBUFFER], dl 
    mov edx, 1
    call Print

    mov edx, 8
    mov eax, edi 
    call PrintHex

    mov ecx, OUTBUFFER 
    mov dl, 0x3A   ; ':'
    mov [OUTBUFFER], dl 
    mov dl, 0x20   ; 0x20 = ' '
    mov [OUTBUFFER], dl 
    mov edx, 2
    call Print
    mov esi, 16

    pop edi

_get_byte:
    push esi
    push edi
    mov ecx, INBUFFER 
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
    mov ecx, OUTBUFFER
    mov [OUTBUFFER], eax
    call Print

    ; Exit
    mov eax, 1          ; exit
    xor ebx, ebx
    int 0x80
