# Implementation of trap entry for RISC-V 64 Supervisor Mode
# This saves the full processor state (context) to the stack before
# calling the Rust trap_handler.

    .global trap_entry
    .section .text.trap_entry
    .align 4 # Traps require 4-byte alignment

trap_entry:
    # Allocate space on the current stack for 32 registers (32 * 8 = 256 bytes)
    addi sp, sp, -256

    # Save general purpose registers 
    # NOTES: 
    #   - We don't save x0 since it is always 0
    #   - x2 (stack pointer) is handled implicitely
    sd ra,  0  * 8(sp)
    sd gp,  2  * 8(sp)
    sd tp,  3  * 8(sp)
    sd t0,  4  * 8(sp)
    sd t1,  5  * 8(sp)
    sd t2,  6  * 8(sp)
    sd s0,  7  * 8(sp)
    sd s1,  8  * 8(sp)
    sd a0,  9  * 8(sp)
    sd a1,  10 * 8(sp)
    sd a2,  11 * 8(sp)
    sd a3,  12 * 8(sp)
    sd a4,  13 * 8(sp)
    sd a5,  14 * 8(sp)
    sd a6,  15 * 8(sp)
    sd a7,  16 * 8(sp)
    sd s2,  17 * 8(sp)
    sd s3,  18 * 8(sp)
    sd s4,  19 * 8(sp)
    sd s5,  20 * 8(sp)
    sd s6,  21 * 8(sp)
    sd s7,  22 * 8(sp)
    sd s8,  23 * 8(sp)
    sd s9,  24 * 8(sp)
    sd s10, 25 * 8(sp)
    sd s11, 26 * 8(sp)
    sd t3,  27 * 8(sp)
    sd t4,  28 * 8(sp)
    sd t5,  29 * 8(sp)
    sd t6,  30 * 8(sp)

    # Call the Rust handler
    # Pass the stack pointer as the first argument (a0) so Rust sees the TrapFrame
    mv a0, sp
    call trap_handler

    # Restore all general purpose registers
    ld ra,  0  * 8(sp)
    ld gp,  2  * 8(sp)
    ld tp,  3  * 8(sp)
    ld t0,  4  * 8(sp)
    ld t1,  5  * 8(sp)
    ld t2,  6  * 8(sp)
    ld s0,  7  * 8(sp)
    ld s1,  8  * 8(sp)
    ld a0,  9  * 8(sp)
    ld a1,  10 * 8(sp)
    ld a2,  11 * 8(sp)
    ld a3,  12 * 8(sp)
    ld a4,  13 * 8(sp)
    ld a5,  14 * 8(sp)
    ld a6,  15 * 8(sp)
    ld a7,  16 * 8(sp)
    ld s2,  17 * 8(sp)
    ld s3,  18 * 8(sp)
    ld s4,  19 * 8(sp)
    ld s5,  20 * 8(sp)
    ld s6,  21 * 8(sp)
    ld s7,  22 * 8(sp)
    ld s8,  23 * 8(sp)
    ld s9,  24 * 8(sp)
    ld s10, 25 * 8(sp)
    ld s11, 26 * 8(sp)
    ld t3,  27 * 8(sp)
    ld t4,  28 * 8(sp)
    ld t5,  29 * 8(sp)
    ld t6,  30 * 8(sp)

    # Cleanup stack pointer and return
    addi sp, sp, 256
    sret
