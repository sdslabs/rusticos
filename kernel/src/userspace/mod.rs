pub mod userspace;

#[naked] //As we will be adding our own unsafe functions here executing our own assembly language we don't need Prologue and Epilogue for the function

// To get into the kernel mode for performing the syscall execution, one must set the registers to the desired syscall number and its parameters and perform a syscall instruction

// Breaking the process in three step (Returning execution to the user program):
//     1) Enable the page table that has this program's memory mapped to the correct virtual addresses
//     2) Setting the cs and ds registers to proper indexes in the GDT to indicate that we are currently in Ring3 or usermode
//     3) Setting the registers for sysretq and iretq operations

pub unsafe fn userspace_prog_1(){
    asm!("\
        nop
        nop
        nop
    "::::"intel");
}

