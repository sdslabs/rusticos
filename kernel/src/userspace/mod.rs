pub mod userspace;

#[naked] //As we will be adding our own unsafe functions here executing our own assembly language we don't need Prologue and Epilogue for the function

// To get into the kernel mode for performing the syscall execution, one must set the registers to the desired syscall number and its parameters and perform a syscall instruction

// Breaking the process in three step (Returning execution to the user program):
//     1) Enable the page table that has this program's memory mapped to the correct virtual addresses
//     2) Setting the cs and ds registers to proper indexes in the GDT to indicate that we are currently in Ring3 or usermode and use the respective segment
//     3) Setting the registers for sysretq and iretq operations

//Obviously you can remove the jmp statement from the program from infinitely looping

pub unsafe fn userspace_prog_1() {
    asm!("\
        start:
        mov rax, 0xCA11
        mov rdi, 10
        mov rsi, 20
        mov rdx, 30
        mov r10, 40
        syscall
        jmp start
    ":::: "volatile", "intel");
}

//The syscall should automatically invoke handlers::process_syscall with the following registers set as rax - syscall address, and rdi rsi rdx r10 are just 4 registers for 4 arguments passed to the syscall. 
//You can increase this by increasing the number of arguments the syscall can handle and setting the respective registers with the appropriate values in the userspace program. :)


//Random user space program which does something (has miniloops) and calls syscalls
#[naked]
pub unsafe fn userspace_prog_1() {
    asm!("\
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
    ");
}