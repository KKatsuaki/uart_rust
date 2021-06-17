    .section ".text.boot"
    .global _start
_start:
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbz x1, 2f
1:
    wfe
    b 1b
2:  // setting stack pointer
    ldr x1, =_start
    mov sp, x1
    // load _rust_start 
    bl _rust_start
    b 1b
