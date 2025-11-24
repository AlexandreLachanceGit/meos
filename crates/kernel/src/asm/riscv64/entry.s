	.global _start
	.extern _STACK_PTR
	.extern _KERNEL_END

	.section .text.boot

_start:	
    la sp, _STACK_PTR

    la t0, _KERNEL_END
    sd t0, 0(t0)

    jal main
	j .
