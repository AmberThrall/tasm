ENTRY Main

; Calls sys_read to read a single byte and stores the output to [ecx]
; Returns the number of bytes read (zero indicates EOF) to eax
GetByte:
    mov eax, 0
    mov [ecx], eax
    mov eax, 3          ; read
    mov ebx, 0          ; stdin
    mov edx, 1          ; count
    int 0x80
    ret

; Clears edx bytes from [ecx]
Clear:
    push eax
    mov eax, 0
_clear_loop:
    mov [ecx], eax
    inc ecx
    dec edx
    jnz _clear_loop
    pop eax
    ret

; Calls sys_write to print edx bytes from our [ecx]
Print:
    mov eax, 4          ; write
    mov ebx, 1          ; stdout
    int 0x80
    ret

; Calls sys_write to print eax in base 16 with a minimum of edx digits
PrintHex:
    xor edi, edi
    mov ebx, 16
_loop:
    push edx
    xor edx, edx
    div ebx
    cmp dl, 9
    jle _skip       ; if dl < 9, then goto _skip
    add dl, 0x07    ; 0x07 = 'A' - '0' - 10
_skip:
    add dl, 0x30    ; '0' = 0x30
    dec ecx
    inc edi
    mov [ecx], dl

    ; Do we keep going?
    pop edx
    dec edx
    cmp edx, 0
    jnz _loop
    cmp eax, 0
    jnz _loop

    ; Finally we print out the byte.
    mov edx, edi
    call Print
    ret

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
