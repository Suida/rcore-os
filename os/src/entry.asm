	.section .text.entry
	.global _start
_start:
	la sp, boot_stack_top
	call rust_main

	# bss is a block if ELF, which only records length, since it will be initialized 
	# as zero value.
	# Here states the launch stack for os
	.section .bss.stack
	.global boot_stack
boot_stack:
	# Stack with size of 16 KB
	.space 4096 * 16
	.global boot_stack_top
boot_stack_top:
	# Stack end

