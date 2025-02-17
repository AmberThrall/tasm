# TASM 

**TASM** is a tiny x86 assembler for Linux targeting System V.
It only contains a subset of the instructions.
There is no linker, your entire source code must be contained in a single source file.

# Installing

From the source directory, simply run
```
cargo install --path .
```

# Usage

TASM takes in a single mandatory argument, the path to the source code.
By default, it assembles your code to `a.out`.
This can be changed by passing the output flag `-o <PATH>`.

TASM's syntax is based on Intel syntax.
Below is a simple example printing "Hello World!" to stdout and exiting.

```asm
entry _start

_msg: 
    db "Hello World!",0xA

_start:
    mov ebx, 1      ; stdout
    mov ecx, _msg   ; what to print
    mov edx, 13     ; message length
    mov eax, 4      ; write
    int 0x80

    mov eax, 1      ; exit
    mov ebx, 0      ; status code 0
    int 0x80
```

More examples can be seen in the `tests` directory.
