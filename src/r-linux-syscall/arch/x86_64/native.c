/*
 * System Calls on x86_64
 *
 * This implements the syscall entries for x86_64. We use the `syscall`
 * instruction, as it is the recommended way to enter the linux kernel as of
 * this time.
 *
 * Arguments are passed as:
 *     Nr: rax
 *     Args: rdi, rsi, rdx, r10, r8, r9
 * Return value is in:
 *     Ret: rax
 *
 * We use GCC-style inline-asm and variable annotations to keep each argument
 * in the correct register before even entering the asm-block. This way, no
 * register movement on our side is required and the compiler can optimize to
 * its liking.
 */

#if defined(__x86_64__)

unsigned long r_linux_asm_syscall0(unsigned long nr) {
	unsigned long r;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall1(unsigned long nr,
				   unsigned long arg0) {
	unsigned long r;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall2(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1) {
	unsigned long r;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0), "S"(arg1)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall3(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2) {
	unsigned long r;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0), "S"(arg1), "d"(arg2)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall4(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2,
				   unsigned long arg3) {
	unsigned long r;
	register unsigned long r10 asm("r10") = arg3;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0), "S"(arg1), "d"(arg2),
		  "r"(r10)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall5(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2,
				   unsigned long arg3,
				   unsigned long arg4) {
	unsigned long r;
	register unsigned long r10 asm("r10") = arg3;
	register unsigned long r8 asm("r8") = arg4;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0), "S"(arg1), "d"(arg2),
		  "r"(r10), "r"(r8)
		: "rcx", "r11", "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall6(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2,
				   unsigned long arg3,
				   unsigned long arg4,
				   unsigned long arg5) {
	unsigned long r;
	register unsigned long r10 asm("r10") = arg3;
	register unsigned long r8 asm("r8") = arg4;
	register unsigned long r9 asm("r9") = arg5;
	asm volatile (
		"syscall"
		: "=a"(r)
		: "a"(nr), "D"(arg0), "S"(arg1), "d"(arg2),
		  "r"(r10), "r"(r8), "r"(r9)
		: "rcx", "r11", "memory"
	);
	return r;
}

#endif
