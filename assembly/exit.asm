section .text
    global _start


_start:
    ; syscall number for exit (60)
    mov rax, 60
    ; exit status (0)
    mov rdi, 1
    ; invoke the syscall
    syscall
