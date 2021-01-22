# rCore-Tutorial 学习笔记

`rCore-Tutorial` 是清华大学操作系统课程的实验项目，讲解了如何使用 rust 语言为 riscv 
编写操作系统，其前身是使用 C 语言实现的 `ucore-plus` 项目。两者为陈渝老师的 
[os_kernel_lab](https://github.com/chyyuu/os_kernel_lab) 项目的两个分支。
`rCore-Tutorial` 在 [教学版](https://github.com/chyyuu/rCore_Tutorial) 中进行了重
构，包含了细致合理的教程文档， 将作为主要学习资料。

### Lab-0，环境配置和项目初始化

...

### Lab-1，中断

最初自学操作系统时就觉得中断的定义、范畴太过模糊，常常只能通过上下文语境进行猜测，
后来熟悉了倒是能猜中，但一旦脱离语境让我自己完整厘清各种关系，就把我自己难住了。
这次就趁这个机会，好好总结一下各名词的确切含义。

#### 中断的种类

根据 Intel 在 *Intel® 64 and IA-32 architectures software developer's manual
volume 1: Basic architecture* 中的解释：

> - An interrupt is an asynchronous event that is typically triggered by an I/O
>   device.
> - An exception is a synchronous event that is generated when the processor
>   detects one or more predefined conditions while executing an instruction. 
>   The IA-32 architecture specifies three classes of exceptions: faults,
>   traps, and aborts. 

广义的中断可以分为狭义中断（后文简称中断）和异常，两种“打断”当前控制流的方法。
中断是指由 I/O 设备触发的异步事件，异常是指当 CPU 发现特定条件满足后触发的同步事件。
异常可以细分为：错误（faults），陷入（traps）和终止（aborts）。

// TODO：完成中断类型的记录

...

#### RISC-V 的中断

