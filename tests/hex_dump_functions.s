;;
;; Functions used in hex_dump.s
;; File is included by hex_dump.s
;;

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

