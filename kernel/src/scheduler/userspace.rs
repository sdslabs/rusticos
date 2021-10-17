#[naked]
pub async unsafe fn userspace_prog_1() {
    asm!(
        "\
        mov rbx, 0xf0000000
        prog1start:
        push 0x595ca11a // keep the syscall number in the stack
        mov rbp, 0x0 // distinct values for each register
        mov rax, 0x1
        mov rcx, 0x3
        mov rdx, 0x4
        mov rdi, 0x6
        mov r8, 0x7
        mov r9, 0x8
        mov r10, 0x9
        mov r11, 0x10
        mov r12, 0x11
        mov r13, 0x12
        mov r14, 0x13
        mov r15, 0x14
        xor rax, rax
        prog1loop:
        inc rax
        cmp rax, 0x4000000
        jnz prog1loop // loop for some milliseconds
        pop rax // pop syscall number from the stack
        inc rbx // increase loop counter
        mov rdi, rsp // first syscall arg is rsp
        mov rsi, rbx // second syscall arg is the loop counter
        syscall // perform the syscall!
        jmp prog1start // do it all over
    "
    );
}

#[naked]
pub unsafe fn userspace_prog_2() {
    asm!(
        "\
        mov rbx, 0
        prog2start:
        push 0x595ca11b // keep the syscall number in the stack
        mov rbp, 0x100 // distinct values for each register
        mov rax, 0x101
        mov rcx, 0x103
        mov rdx, 0x104
        mov rdi, 0x106
        mov r8, 0x107
        mov r9, 0x108
        mov r10, 0x109
        mov r11, 0x110
        mov r12, 0x111
        mov r13, 0x112
        mov r14, 0x113
        mov r15, 0x114
        xor rax, rax
        prog2loop:
        inc rax
        cmp rax, 0x4000000
        jnz prog2loop // loop for some milliseconds
        pop rax // pop syscall number from the stack
        inc rbx // increase loop counter
        mov rdi, rsp // first syscall arg is rsp
        mov rsi, rbx // second syscall arg is the loop counter
        syscall // perform the syscall!
        jmp prog2start // do it all over
    "
    );
}
