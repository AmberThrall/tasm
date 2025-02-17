ENTRY _start

; Calls sys_read to read a single byte and stores the output to ecx
GetByte:
    mov eax, 0
    mov [ecx], eax        ; clear the buffer
    mov eax, 3          ; read
    mov ebx, 0          ; stdin
    mov edx, 1          ; count
    int 0x80
    ret

; Calls sys_write to print a single byte from ecx
PrintByte:
    mov eax, 4          ; write
    mov ebx, 1          ; stdout
    int 0x80
    ret

_start:
    mov ecx, _buffer
    call GetByte

    ; Check if the input is done, i.e., [ecx] is zero
    mov eax, [ecx]
    cmp eax, 0
    jz _exit

    call PrintByte
    jmp _start

_exit:
    ; Exit
    mov eax, 1          ; exit
    xor ebx, ebx
    int 0x80

_buffer:
