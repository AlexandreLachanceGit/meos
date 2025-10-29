# Implementation of context switching for RISC-V 64
# Info on callee-saved registers :
# https://riscv.org/wp-content/uploads/2024/12/riscv-calling.pdf

    .global switch_context        
    .section .text.switch_context 
    .type switch_context, @function

switch_context: # (a0 = prev_sp, a1 = next_sp)
    # Save callee-saved registers
    addi sp, sp, -13 * 8
    sd ra,  0  * 8(sp) # Return address

    ## Int registers
    sd s0,  1  * 8(sp)
    sd s1,  2  * 8(sp)
    sd s2,  3  * 8(sp)
    sd s3,  4  * 8(sp)
    sd s4,  5  * 8(sp)
    sd s5,  6  * 8(sp)
    sd s6,  7  * 8(sp)
    sd s7,  8  * 8(sp)
    sd s8,  9  * 8(sp)
    sd s9,  10 * 8(sp)
    sd s10, 11 * 8(sp)
    sd s11, 12 * 8(sp)

    # Switch the stack pointer 
    sd sp, 0(a0) # *prev_sp = sp
    ld sp, 0(a1) # sp = *next_sp

    # Load callee-saved registers
    ld ra,  0  * 8(sp) # Return address

    ## Int registers
    ld s0,  1  * 8(sp)
    ld s1,  2  * 8(sp)
    ld s2,  3  * 8(sp)
    ld s3,  4  * 8(sp)
    ld s4,  5  * 8(sp)
    ld s5,  6  * 8(sp)
    ld s6,  7  * 8(sp)
    ld s7,  8  * 8(sp)
    ld s8,  9  * 8(sp)
    ld s9,  10 * 8(sp)
    ld s10, 11 * 8(sp)
    ld s11, 12 * 8(sp)

    # Cleanup stack pointer
    addi sp, sp, 13 * 8
    ret
