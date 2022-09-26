pub mod userspace;

#[naked] //As we will be adding our own unsafe functions here executing our own assembly language we don't need Prologue and Epilogue for the function

// To get into the kernel mode for performing the syscall execution, one must set the registers to the desired syscall number and its parameters and perform a syscall instruction

// Breaking the process in three step (Returning execution to the user program):
//     1) Enable the page table that has this program's memory mapped to the correct virtual addresses
//     2) Setting the cs and ds registers to proper indexes in the GDT to indicate that we are currently in Ring3 or usermode
//     3) Setting the registers for sysretq and iretq operations

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
