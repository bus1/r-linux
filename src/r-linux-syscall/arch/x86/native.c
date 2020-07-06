/*
 * System Calls on x86
 *
 * This implements the syscall entries for x86. We use the `int$0x80` software
 * interrupt to enter the kernel. It would be much faster to use the VDSO entry
 * point, but it does require access to `%gs` and the TLS mappings, and thus is
 * left for future improvements.
 *
 * Arguments are passed as:
 *     Nr: eax
 *     Args: ebx, ecx, edx, esi, edi, ebp
 * Return value is in:
 *     Ret: eax
 *
 * We use GCC-style inline-asm and variable annotations to keep each argument
 * in the correct register before even entering the asm-block. This way, no
 * register movement on our side is required and the compiler can optimize to
 * its liking.
 */

#if defined(__i386__)

unsigned long r_linux_asm_syscall0(unsigned long nr) {
	unsigned long r;
	asm volatile (
		"int $0x80"
		: "=a"(r)
		: "a"(nr)
		: "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall1(unsigned long nr,
				   unsigned long arg0) {
	unsigned long r;
	asm volatile (
		"int $0x80"
		: "=a"(r)
		: "a"(nr), "b"(arg0)
		: "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall2(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1) {
	unsigned long r;
	asm volatile (
		"int $0x80"
		: "=a"(r)
		: "a"(nr), "b"(arg0), "c"(arg1)
		: "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall3(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2) {
	unsigned long r;
	asm volatile (
		"int $128"
		: "=a"(r)
		: "a"(nr), "b"(arg0), "c"(arg1), "d"(arg2)
		: "memory"
	);
	return r;
}

unsigned long r_linux_asm_syscall4(unsigned long nr,
				   unsigned long arg0,
				   unsigned long arg1,
				   unsigned long arg2,
				   unsigned long arg3) {
	unsigned long r;
	asm volatile (
		"int $128"
		: "=a"(r)
		: "a"(nr), "b"(arg0), "c"(arg1), "d"(arg2),
		  "S"(arg3)
		: "memory"
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
	asm volatile (
		"int $128"
		: "=a"(r)
		: "a"(nr), "b"(arg0), "c"(arg1), "d"(arg2),
		  "S"(arg3), "D"(arg4)
		: "memory"
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
	asm volatile (
		/*
		 * We need to pass `arg5` in `ebp`. We cannot easily get this
		 * in another register (there is no `ebp` constraint for inline
		 * asm). Instead, we use the generic `g` constraint. We also
		 * immediately save it on the stack, because it might be `esp`
		 * based and thus invalid as soon as we modify `esp` to save
		 * `ebp`.
		 */
		"pushl %7;"
		"push %%ebp;"
		"mov 4(%%esp),%%ebp;"
		"int $128;"
		"pop %%ebp;"
		"add $4,%%esp"
		: "=a"(r)
		: "a"(nr), "b"(arg0), "c"(arg1), "d"(arg2),
		  "S"(arg3), "D"(arg4), "g"(arg5)
		: "memory"
	);
	return r;
}

#endif
