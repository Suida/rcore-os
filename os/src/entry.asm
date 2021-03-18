	.section .text.entry
	.global _start
_start:
	# 计算 boot_page_table 的物理页号
	lui t0, %hi(boot_page_table)
	li t1, 0xffffffff00000000
	sub t0, t0, t1
	srli t0, 12
	# 8 << 60 是 satp 中使能 Sv39 的记号
	li t1, (8 << 60)
	or t0, t0, t1
	# 写入 satp，刷新 TLB
	csrw satp, t0
	sfence.vma

	#加载栈地址
	lui sp, %hi(boot_stack_top)
	addi sp, sp, %lo(boot_stack_top)

	# 跳转到 rust_main
	lui t0, %hi(rust_main)
	addi t0, %lo(rust_main)
	jr t0

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

	# 初始内核映射所用的页表
	.section .data
	.align 12
boot_page_table:
	.quad 0
	.quad 0
	# 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
	.quad (0x8000_0000 << 10) | 0xcf
	.zero 507 * 8
	# 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
	.quad (0x8000_0000 << 10) | 0xcf
	.quad 0
