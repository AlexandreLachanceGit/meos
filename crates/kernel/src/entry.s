	.global _start
	.extern _STACK_PTR
	.extern _HEAP_START
	.extern _HEAP_END

	.section .text.boot

_start:	la sp, _STACK_PTR

    la t0, _HEAP_START
    sd t0, 0(t0)

    la t0, _HEAP_END
    sd t0, 0(t0)

    jal main
	j .
