	.text
	.file	"lay_wayland.aa31e2da4b4996c0-cgu.0"
	.section	.text._ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE,@function
_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	subq	$232, %rsp
	.cfi_def_cfa_offset 272
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r14
	movq	%rsi, %r15
	movq	%rdi, %rbx
	leaq	8(%rsp), %rdi
	movl	$1, %esi
	movl	$1, %edx
	callq	*_ZN3std3sys3pal4unix3net6Socket7new_raw17h992a68b5f59f7328E@GOTPCREL(%rip)
	cmpl	$0, 8(%rsp)
	je	.LBB0_4
	movq	16(%rsp), %rax
	movq	%rax, 8(%rbx)
	movl	$1, (%rbx)
	jmp	.LBB0_12
.LBB0_4:
	movl	12(%rsp), %ebp
.Ltmp0:
	leaq	8(%rsp), %rdi
	movq	%r15, %rsi
	movq	%r14, %rdx
	callq	*_ZN3std2os4unix3net4addr11sockaddr_un17he8d931dc73c00344E@GOTPCREL(%rip)
.Ltmp1:
	cmpl	$0, 8(%rsp)
	je	.LBB0_6
	movq	16(%rsp), %rax
.LBB0_10:
	movq	%rax, 8(%rbx)
	movl	$1, (%rbx)
	movl	%ebp, %edi
	callq	*close@GOTPCREL(%rip)
	jmp	.LBB0_12
.LBB0_6:
	movl	12(%rsp), %eax
	movq	16(%rsp), %rcx
	movups	24(%rsp), %xmm0
	movaps	%xmm0, 128(%rsp)
	movups	40(%rsp), %xmm0
	movaps	%xmm0, 144(%rsp)
	movups	56(%rsp), %xmm0
	movaps	%xmm0, 160(%rsp)
	movups	72(%rsp), %xmm0
	movaps	%xmm0, 176(%rsp)
	movups	88(%rsp), %xmm0
	movaps	%xmm0, 192(%rsp)
	movups	104(%rsp), %xmm0
	movaps	%xmm0, 208(%rsp)
	movl	120(%rsp), %edx
	movl	%edx, 224(%rsp)
	movl	124(%rsp), %edx
	movzwl	224(%rsp), %esi
	movw	%si, 116(%rsp)
	movaps	208(%rsp), %xmm0
	movups	%xmm0, 100(%rsp)
	movaps	192(%rsp), %xmm0
	movups	%xmm0, 84(%rsp)
	movaps	128(%rsp), %xmm0
	movaps	144(%rsp), %xmm1
	movaps	160(%rsp), %xmm2
	movaps	176(%rsp), %xmm3
	movups	%xmm3, 68(%rsp)
	movups	%xmm2, 52(%rsp)
	movups	%xmm1, 36(%rsp)
	movups	%xmm0, 20(%rsp)
	movl	%eax, 8(%rsp)
	movq	%rcx, 12(%rsp)
	leaq	8(%rsp), %rsi
	movl	%ebp, %edi
	callq	*connect@GOTPCREL(%rip)
	cmpl	$-1, %eax
	je	.LBB0_7
	movl	%ebp, 4(%rbx)
	movl	$0, (%rbx)
.LBB0_12:
	addq	$232, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB0_7:
	.cfi_def_cfa_offset 272
.Ltmp2:
	callq	*_ZN3std3sys3pal4unix2os5errno17h2141573766d4db30E@GOTPCREL(%rip)
.Ltmp3:
	shlq	$32, %rax
	orq	$2, %rax
	jmp	.LBB0_10
.LBB0_2:
.Ltmp4:
	movq	%rax, %rbx
.Ltmp5:
	movl	%ebp, %edi
	callq	*close@GOTPCREL(%rip)
.Ltmp6:
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
.LBB0_13:
.Ltmp7:
	callq	*_ZN4core9panicking16panic_in_cleanup17hd62aa59d1fda1c9fE@GOTPCREL(%rip)
.Lfunc_end0:
	.size	_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE, .Lfunc_end0-_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE
	.cfi_endproc
	.section	.gcc_except_table._ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE,"a",@progbits
	.p2align	2, 0x0
GCC_except_table0:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Lfunc_begin0-.Lfunc_begin0
	.uleb128 .Ltmp0-.Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Ltmp2-.Ltmp1
	.byte	0
	.byte	0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.uleb128 .Ltmp3-.Ltmp2
	.uleb128 .Ltmp4-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp5-.Lfunc_begin0
	.uleb128 .Ltmp6-.Ltmp5
	.uleb128 .Ltmp7-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp6-.Lfunc_begin0
	.uleb128 .Lfunc_end0-.Ltmp6
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E,@function
_ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E:
	.cfi_startproc
	retq
.Lfunc_end1:
	.size	_ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E, .Lfunc_end1-_ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E
	.cfi_endproc

	.section	".text._ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E,@function
_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	pushq	%rax
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	cmpl	$0, (%rdi)
	je	.LBB2_11
	movq	8(%rdi), %rax
	movl	%eax, %ecx
	andl	$3, %ecx
	leaq	-2(%rcx), %rdx
	cmpq	$2, %rdx
	jb	.LBB2_10
	testq	%rcx, %rcx
	jne	.LBB2_3
.LBB2_10:
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB2_11:
	.cfi_def_cfa_offset 48
	movl	4(%rdi), %edi
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	jmpq	*close@GOTPCREL(%rip)
.LBB2_3:
	.cfi_def_cfa_offset 48
	leaq	-1(%rax), %rbx
	movq	-1(%rax), %r14
	movq	7(%rax), %r12
.Ltmp8:
	movq	%r14, %rdi
	callq	*(%r12)
.Ltmp9:
	movq	8(%r12), %rsi
	testq	%rsi, %rsi
	je	.LBB2_6
	movq	16(%r12), %rdx
	movq	%r14, %rdi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB2_6:
	movl	$24, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB2_7:
	.cfi_def_cfa_offset 48
.Ltmp10:
	movq	%rax, %r15
	movq	8(%r12), %rsi
	testq	%rsi, %rsi
	je	.LBB2_9
	movq	16(%r12), %rdx
	movq	%r14, %rdi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB2_9:
	movl	$24, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	callq	*__rust_dealloc@GOTPCREL(%rip)
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end2:
	.size	_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E, .Lfunc_end2-_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E","a",@progbits
	.p2align	2, 0x0
GCC_except_table2:
.Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Lfunc_begin1-.Lfunc_begin1
	.uleb128 .Ltmp8-.Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 .Ltmp8-.Lfunc_begin1
	.uleb128 .Ltmp9-.Ltmp8
	.uleb128 .Ltmp10-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp9-.Lfunc_begin1
	.uleb128 .Lfunc_end2-.Ltmp9
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2, 0x0

	.section	".text._ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E","ax",@progbits
	.globl	_ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E
	.p2align	4, 0x90
	.type	_ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E,@function
_ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$152, %rsp
	.cfi_def_cfa_offset 208
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	_ZN3log20MAX_LOG_LEVEL_FILTER17h1f018aa8b0224278E@GOTPCREL(%rip), %r13
	movq	(%r13), %rax
	cmpq	$3, %rax
	jb	.LBB3_2
	leaq	.L__unnamed_1(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	$8, 32(%rsp)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 40(%rsp)
	leaq	.L__unnamed_2(%rip), %rdx
	leaq	16(%rsp), %rdi
	movl	$3, %esi
	movl	$69, %ecx
	xorl	%r8d, %r8d
	callq	*_ZN3log13__private_api8log_impl17h309cd7e7e772a81dE@GOTPCREL(%rip)
.LBB3_2:
	movabsq	$-9223372036854775808, %r14
	leaq	.L__unnamed_3(%rip), %rsi
	leaq	128(%rsp), %rdi
	movl	$15, %edx
	callq	*_ZN3std3env7_var_os17hc98e9a7d243dd731E@GOTPCREL(%rip)
	movq	128(%rsp), %rbx
	cmpq	%r14, %rbx
	je	.LBB3_50
	movq	136(%rsp), %rbp
	movq	144(%rsp), %r15
.Ltmp11:
	leaq	.L__unnamed_4(%rip), %rsi
	leaq	104(%rsp), %rdi
	movl	$15, %edx
	callq	*_ZN3std3env7_var_os17hc98e9a7d243dd731E@GOTPCREL(%rip)
.Ltmp12:
	movq	104(%rsp), %r12
	cmpq	%r14, %r12
	je	.LBB3_29
	movq	112(%rsp), %r13
	movq	120(%rsp), %r8
.Ltmp13:
	movq	%rbp, %rsi
	leaq	64(%rsp), %rbp
	movq	%rbp, %rdi
	movq	%rsi, 8(%rsp)
	movq	%r15, %rdx
	movq	%r13, %rcx
	callq	*_ZN3std4path4Path5_join17h28e226e2326aba56E@GOTPCREL(%rip)
.Ltmp14:
	testq	%r12, %r12
	je	.LBB3_8
	movl	$1, %edx
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_8:
	movq	_ZN3log20MAX_LOG_LEVEL_FILTER17h1f018aa8b0224278E@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	cmpq	$3, %rax
	jb	.LBB3_10
	movq	%rbp, 88(%rsp)
	movq	_ZN55_$LT$std..path..PathBuf$u20$as$u20$core..fmt..Debug$GT$3fmt17he4c695e9c5a18a52E@GOTPCREL(%rip), %rax
	movq	%rax, 96(%rsp)
	leaq	.L__unnamed_5(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	$0, 48(%rsp)
	leaq	88(%rsp), %rax
	movq	%rax, 32(%rsp)
	movq	$1, 40(%rsp)
.Ltmp16:
	leaq	.L__unnamed_2(%rip), %rdx
	leaq	16(%rsp), %rdi
	movl	$3, %esi
	movl	$76, %ecx
	xorl	%r8d, %r8d
	callq	*_ZN3log13__private_api8log_impl17h309cd7e7e772a81dE@GOTPCREL(%rip)
.Ltmp17:
.LBB3_10:
	movq	64(%rsp), %rbp
	movq	72(%rsp), %r12
	movq	80(%rsp), %rdx
.Ltmp19:
	leaq	16(%rsp), %rdi
	movq	%r12, %rsi
	callq	_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE
.Ltmp20:
	movl	16(%rsp), %r14d
	testl	%r14d, %r14d
	je	.LBB3_12
	movq	24(%rsp), %rax
	movq	%rax, 96(%rsp)
	movl	$1, %eax
	movl	%eax, 88(%rsp)
	testq	%rbp, %rbp
	je	.LBB3_26
.LBB3_25:
	movl	$1, %edx
	movq	%r12, %rdi
	movq	%rbp, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_26:
	testl	%r14d, %r14d
	je	.LBB3_36
.Ltmp22:
	leaq	88(%rsp), %rdi
	movq	8(%rsp), %rbp
	callq	_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E
.Ltmp23:
	movq	_ZN3log20MAX_LOG_LEVEL_FILTER17h1f018aa8b0224278E@GOTPCREL(%rip), %r13
.LBB3_29:
.Ltmp25:
	leaq	.L__unnamed_6(%rip), %rcx
	leaq	64(%rsp), %r12
	movl	$9, %r8d
	movq	%r12, %rdi
	movq	%rbp, %rsi
	movq	%r15, %rdx
	callq	*_ZN3std4path4Path5_join17h28e226e2326aba56E@GOTPCREL(%rip)
.Ltmp26:
	movq	(%r13), %rax
	cmpq	$3, %rax
	jb	.LBB3_32
	movq	%r12, 104(%rsp)
	movq	_ZN55_$LT$std..path..PathBuf$u20$as$u20$core..fmt..Debug$GT$3fmt17he4c695e9c5a18a52E@GOTPCREL(%rip), %rax
	movq	%rax, 112(%rsp)
	leaq	.L__unnamed_7(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	$0, 48(%rsp)
	leaq	104(%rsp), %rax
	movq	%rax, 32(%rsp)
	movq	$1, 40(%rsp)
.Ltmp27:
	leaq	.L__unnamed_2(%rip), %rdx
	leaq	16(%rsp), %rdi
	movl	$3, %esi
	movl	$87, %ecx
	xorl	%r8d, %r8d
	callq	*_ZN3log13__private_api8log_impl17h309cd7e7e772a81dE@GOTPCREL(%rip)
.Ltmp28:
.LBB3_32:
.Ltmp29:
	leaq	16(%rsp), %rdi
	movq	%rbp, %rsi
	movq	%r15, %rdx
	callq	_ZN3std2os4unix3net6stream10UnixStream7connect17he1a27f7f9084413fE
.Ltmp30:
	cmpl	$0, 16(%rsp)
	je	.LBB3_34
	movq	24(%rsp), %rax
	movq	%rax, 112(%rsp)
	movl	$1, 104(%rsp)
	movq	64(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB3_47
	movq	72(%rsp), %rdi
	movl	$1, %edx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_47:
.Ltmp32:
	leaq	104(%rsp), %rdi
	callq	_ZN4core3ptr96drop_in_place$LT$core..result..Result$LT$lay_wayland..Connection$C$std..io..error..Error$GT$$GT$17ha20a3a60b91b0f49E
.Ltmp33:
	testq	%rbx, %rbx
	je	.LBB3_50
	movl	$1, %edx
	movq	%rbp, %rdi
	movq	%rbx, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_50:
	movq	(%r13), %rax
	movl	$257, %r13d
	testq	%rax, %rax
	je	.LBB3_40
	leaq	.L__unnamed_8(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	$8, 32(%rsp)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 40(%rsp)
	leaq	.L__unnamed_2(%rip), %rdx
	leaq	16(%rsp), %rdi
	movl	$1, %esi
	movl	$96, %ecx
	xorl	%r8d, %r8d
	callq	*_ZN3log13__private_api8log_impl17h309cd7e7e772a81dE@GOTPCREL(%rip)
	jmp	.LBB3_40
.LBB3_34:
	movl	20(%rsp), %r13d
	movq	64(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB3_37
	movq	72(%rsp), %rdi
	movl	$1, %edx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_37:
	testq	%rbx, %rbx
	je	.LBB3_39
.LBB3_38:
	movl	$1, %edx
	movq	%rbp, %rdi
	movq	%rbx, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_39:
	shlq	$32, %r13
	orq	$256, %r13
.LBB3_40:
	movq	%r13, %rax
	addq	$152, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB3_12:
	.cfi_def_cfa_offset 208
	movl	20(%rsp), %r13d
	movl	%r13d, 92(%rsp)
	xorl	%eax, %eax
	movl	%eax, 88(%rsp)
	testq	%rbp, %rbp
	jne	.LBB3_25
	jmp	.LBB3_26
.LBB3_36:
	movq	8(%rsp), %rbp
	testq	%rbx, %rbx
	jne	.LBB3_38
	jmp	.LBB3_39
.LBB3_41:
.Ltmp18:
	jmp	.LBB3_42
.LBB3_18:
.Ltmp24:
	jmp	.LBB3_14
.LBB3_21:
.Ltmp21:
	movq	%rax, %r15
	testq	%rbp, %rbp
	je	.LBB3_15
	movl	$1, %edx
	movq	%r12, %rdi
	movq	%rbp, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB3_15
.LBB3_19:
.Ltmp15:
	movq	%rax, %r15
	testq	%r12, %r12
	je	.LBB3_15
	movl	$1, %edx
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB3_15
.LBB3_44:
.Ltmp31:
	movq	%rbp, 8(%rsp)
.LBB3_42:
	movq	%rax, %r15
	movq	64(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB3_15
	movq	72(%rsp), %rdi
	movl	$1, %edx
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB3_15
.LBB3_13:
.Ltmp34:
	movq	%rbp, 8(%rsp)
.LBB3_14:
	movq	%rax, %r15
.LBB3_15:
	testq	%rbx, %rbx
	je	.LBB3_17
	movl	$1, %edx
	movq	8(%rsp), %rdi
	movq	%rbx, %rsi
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB3_17:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end3:
	.size	_ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E, .Lfunc_end3-_ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E
	.cfi_endproc
	.section	".gcc_except_table._ZN11lay_wayland10connection41_$LT$impl$u20$lay_wayland..Connection$GT$8from_env17haf32bf849a41ffd9E","a",@progbits
	.p2align	2, 0x0
GCC_except_table3:
.Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Lfunc_begin2-.Lfunc_begin2
	.uleb128 .Ltmp11-.Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 .Ltmp11-.Lfunc_begin2
	.uleb128 .Ltmp12-.Ltmp11
	.uleb128 .Ltmp34-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp13-.Lfunc_begin2
	.uleb128 .Ltmp14-.Ltmp13
	.uleb128 .Ltmp15-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp16-.Lfunc_begin2
	.uleb128 .Ltmp17-.Ltmp16
	.uleb128 .Ltmp18-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp19-.Lfunc_begin2
	.uleb128 .Ltmp20-.Ltmp19
	.uleb128 .Ltmp21-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp22-.Lfunc_begin2
	.uleb128 .Ltmp23-.Ltmp22
	.uleb128 .Ltmp24-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp25-.Lfunc_begin2
	.uleb128 .Ltmp26-.Ltmp25
	.uleb128 .Ltmp34-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp27-.Lfunc_begin2
	.uleb128 .Ltmp30-.Ltmp27
	.uleb128 .Ltmp31-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp32-.Lfunc_begin2
	.uleb128 .Ltmp33-.Ltmp32
	.uleb128 .Ltmp34-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp33-.Lfunc_begin2
	.uleb128 .Lfunc_end3-.Ltmp33
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2, 0x0

	.section	".text._ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE","ax",@progbits
	.globl	_ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE
	.p2align	4, 0x90
	.type	_ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE,@function
_ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE:
	.cfi_startproc
	movq	%rdi, %rax
	xorl	%ecx, %ecx
	cmpq	$8, %rdx
	jb	.LBB4_2
	movl	(%rsi), %ecx
	movl	%ecx, 4(%rax)
	movd	4(%rsi), %xmm0
	pshuflw	$225, %xmm0, %xmm0
	movd	%xmm0, 8(%rax)
	movl	$1, %ecx
.LBB4_2:
	movl	%ecx, (%rax)
	retq
.Lfunc_end4:
	.size	_ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE, .Lfunc_end4-_ZN11lay_wayland7request9interface38_$LT$impl$u20$lay_wayland..Message$GT$10from_bytes17h08e8363db43dbe6eE
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI5_0:
	.long	0x3b800000
	.section	".text._ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E","ax",@progbits
	.globl	_ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E
	.p2align	4, 0x90
	.type	_ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E,@function
_ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$40, %rsp
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	leaq	.L__unnamed_9(%rip), %rdx
	leaq	16(%rsp), %r14
	movl	$7, %ecx
	movq	%r14, %rdi
	callq	*_ZN4core3fmt9Formatter11debug_tuple17hc2604b6111ac87dcE@GOTPCREL(%rip)
	cvtsi2ssl	(%rbx), %xmm0
	mulss	.LCPI5_0(%rip), %xmm0
	movss	%xmm0, 12(%rsp)
	leaq	.L__unnamed_10(%rip), %rdx
	leaq	12(%rsp), %rsi
	movq	%r14, %rdi
	callq	*_ZN4core3fmt8builders10DebugTuple5field17h5876bbc65916c723E@GOTPCREL(%rip)
	movq	%rax, %rdi
	callq	*_ZN4core3fmt8builders10DebugTuple6finish17he051b7853908cccbE@GOTPCREL(%rip)
	addq	$40, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E, .Lfunc_end5-_ZN11lay_wayland5utils67_$LT$impl$u20$core..fmt..Debug$u20$for$u20$lay_wayland..WlFixed$GT$3fmt17hdd29d12f75856cb9E
	.cfi_endproc

	.section	".text._ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E","ax",@progbits
	.globl	_ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E
	.p2align	4, 0x90
	.type	_ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E,@function
_ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E:
	.cfi_startproc
	cmpq	$5, %rsi
	jbe	.LBB6_2
	xorl	%eax, %eax
	shlq	$16, %rcx
	movzwl	%ax, %eax
	orq	%rcx, %rax
	retq
.LBB6_2:
	cmpq	$3, %rsi
	jbe	.LBB6_5
	movzwl	2(%rdi), %eax
	movzwl	(%rdi), %ecx
	shll	$16, %ecx
	orl	%eax, %ecx
	movw	$1, %ax
	shlq	$16, %rcx
	movzwl	%ax, %eax
	orq	%rcx, %rax
	retq
.LBB6_5:
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_11(%rip), %rdx
	movl	$4, %edi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.Lfunc_end6:
	.size	_ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E, .Lfunc_end6-_ZN126_$LT$core..option..Option$LT$$u5b$u16$u3b$$u20$2$u5d$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17ha8da8c898bcbec58E
	.cfi_endproc

	.section	".text._ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E","ax",@progbits
	.globl	_ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E
	.p2align	4, 0x90
	.type	_ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E,@function
_ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpq	$5, %rsi
	ja	.LBB7_3
	cmpq	$3, %rsi
	jbe	.LBB7_2
	movzwl	2(%rdi), %ecx
	movzwl	(%rdi), %eax
	shll	$16, %eax
	orl	%ecx, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB7_3:
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_12(%rip), %rdi
	callq	*_ZN4core6option13unwrap_failed17h4b4353bf890a85dfE@GOTPCREL(%rip)
.LBB7_2:
	leaq	.L__unnamed_11(%rip), %rdx
	movl	$4, %edi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.Lfunc_end7:
	.size	_ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E, .Lfunc_end7-_ZN98_$LT$$u5b$u16$u3b$$u20$2$u5d$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1873f940ed2c0782E
	.cfi_endproc

	.section	".text._ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE","ax",@progbits
	.globl	_ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE
	.p2align	4, 0x90
	.type	_ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE,@function
_ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE:
	.cfi_startproc
	cmpq	$5, %rsi
	jbe	.LBB8_2
	xorl	%edx, %edx
	jmp	.LBB8_4
.LBB8_2:
	cmpq	$3, %rsi
	jbe	.LBB8_5
	movzwl	2(%rdi), %eax
	movzwl	(%rdi), %ecx
	shll	$16, %ecx
	orl	%eax, %ecx
	movw	$1, %dx
.LBB8_4:
	movl	%ecx, %edi
	shrl	$16, %edi
	shlq	$32, %rdi
	shll	$16, %ecx
	xorl	%eax, %eax
	cmpq	$6, %rsi
	cmovbq	%rcx, %rax
	movzwl	%dx, %ecx
	orq	%rdi, %rax
	orq	%rcx, %rax
	retq
.LBB8_5:
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_11(%rip), %rdx
	movl	$4, %edi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.Lfunc_end8:
	.size	_ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE, .Lfunc_end8-_ZN119_$LT$core..option..Option$LT$$LP$u16$C$u16$RP$$GT$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h1c65153b9c53ac7aE
	.cfi_endproc

	.section	".text._ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E","ax",@progbits
	.globl	_ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E
	.p2align	4, 0x90
	.type	_ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E,@function
_ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpq	$5, %rsi
	ja	.LBB9_3
	cmpq	$3, %rsi
	jbe	.LBB9_2
	movzwl	(%rdi), %edx
	movzwl	2(%rdi), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB9_3:
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_13(%rip), %rdi
	callq	*_ZN4core6option13unwrap_failed17h4b4353bf890a85dfE@GOTPCREL(%rip)
.LBB9_2:
	leaq	.L__unnamed_11(%rip), %rdx
	movl	$4, %edi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.Lfunc_end9:
	.size	_ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E, .Lfunc_end9-_ZN91_$LT$$LP$u16$C$u16$RP$$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$$u5b$u8$u5d$$GT$$GT$9from_word17h99340c5de6188a12E
	.cfi_endproc

	.section	".text._ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E","ax",@progbits
	.globl	_ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E
	.p2align	4, 0x90
	.type	_ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E,@function
_ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E:
	.cfi_startproc
	cmpq	$4, %rsi
	jne	.LBB10_2
	movl	(%rdi), %eax
	retq
.LBB10_2:
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_14(%rip), %rdi
	callq	*_ZN4core6option13unwrap_failed17h4b4353bf890a85dfE@GOTPCREL(%rip)
.Lfunc_end10:
	.size	_ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E, .Lfunc_end10-_ZN67_$LT$u32$u20$as$u20$lay_wayland..utils..FromWords$LT$$RF$u8$GT$$GT$9from_word17h26a78d3785c53275E
	.cfi_endproc

	.section	".text._ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE","ax",@progbits
	.globl	_ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE
	.p2align	4, 0x90
	.type	_ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE,@function
_ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$24, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	movzwl	4(%rsi), %eax
	leaq	-8(%rax), %rsi
	cmpq	%rcx, %rsi
	jbe	.LBB11_1
	movabsq	$-9223372036854775808, %rax
	movq	%rax, (%rbx)
	jmp	.LBB11_9
.LBB11_1:
	cmpw	$8, %ax
	jb	.LBB11_10
	cmpq	%rcx, %rax
	ja	.LBB11_3
	cmpq	$3, %rsi
	jbe	.LBB11_11
	movl	8(%rdx), %eax
	cmpq	$4, %rax
	jb	.LBB11_12
	cmpq	%rax, %rsi
	jb	.LBB11_13
	addq	$-4, %rax
	addq	$12, %rdx
	movq	%rsp, %r14
	movq	%r14, %rdi
	movq	%rdx, %rsi
	movq	%rax, %rdx
	callq	*_ZN5alloc6string6String15from_utf8_lossy17h3876e30350249df0E@GOTPCREL(%rip)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	*_ZN98_$LT$alloc..string..String$u20$as$u20$core..convert..From$LT$alloc..borrow..Cow$LT$str$GT$$GT$$GT$4from17h838f2a27b6de96ebE@GOTPCREL(%rip)
.LBB11_9:
	movq	%rbx, %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB11_10:
	.cfi_def_cfa_offset 48
	leaq	.L__unnamed_15(%rip), %rdx
	movl	$8, %edi
	movq	%rax, %rsi
	callq	*_ZN4core5slice5index22slice_index_order_fail17h375f84510c422e93E@GOTPCREL(%rip)
.LBB11_3:
	leaq	.L__unnamed_15(%rip), %rdx
	movq	%rax, %rdi
	movq	%rcx, %rsi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.LBB11_11:
	leaq	.L__unnamed_16(%rip), %rdx
	movl	$4, %edi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.LBB11_12:
	leaq	.L__unnamed_17(%rip), %rdx
	movl	$4, %edi
	movq	%rax, %rsi
	callq	*_ZN4core5slice5index22slice_index_order_fail17h375f84510c422e93E@GOTPCREL(%rip)
.LBB11_13:
	leaq	.L__unnamed_17(%rip), %rdx
	movq	%rax, %rdi
	callq	*_ZN4core5slice5index24slice_end_index_len_fail17h70ffde91723d2a46E@GOTPCREL(%rip)
.Lfunc_end11:
	.size	_ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE, .Lfunc_end11-_ZN11lay_wayland5utils39_$LT$impl$u20$lay_wayland..WlString$GT$8from_buf17h14e1ba227796a93bE
	.cfi_endproc

	.section	".text._ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E","ax",@progbits
	.globl	_ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E
	.p2align	4, 0x90
	.type	_ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E,@function
_ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E:
	.cfi_startproc
	subq	$88, %rsp
	.cfi_def_cfa_offset 96
	movzwl	6(%rdi), %eax
	movl	4(%rdi), %ecx
	shll	$16, %ecx
	orl	%eax, %ecx
	movl	%ecx, 4(%rsp)
	movq	%rdi, 8(%rsp)
	movq	_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17hadfaabd8d73002dfE@GOTPCREL(%rip), %rax
	movq	%rax, 16(%rsp)
	leaq	4(%rsp), %rcx
	movq	%rcx, 24(%rsp)
	movq	%rax, 32(%rsp)
	leaq	.L__unnamed_18(%rip), %rax
	movq	%rax, 40(%rsp)
	movq	$2, 48(%rsp)
	movq	$0, 72(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 56(%rsp)
	movq	$2, 64(%rsp)
	movq	32(%rsi), %rdi
	movq	40(%rsi), %rsi
	leaq	40(%rsp), %rdx
	callq	*_ZN4core3fmt5write17hc090a2ffd6b28c4aE@GOTPCREL(%rip)
	addq	$88, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end12:
	.size	_ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E, .Lfunc_end12-_ZN11lay_wayland5utils70_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$lay_wayland..Message$GT$3fmt17h3116ae42beea06c3E
	.cfi_endproc

	.type	.L__unnamed_19,@object
	.section	.rodata..L__unnamed_19,"a",@progbits
.L__unnamed_19:
	.ascii	"Attempt to connecting to wayland socket"
	.size	.L__unnamed_19, 39

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.quad	.L__unnamed_19
	.asciz	"'\000\000\000\000\000\000"
	.size	.L__unnamed_1, 16

	.type	.L__unnamed_20,@object
	.section	.rodata..L__unnamed_20,"a",@progbits
.L__unnamed_20:
	.ascii	"connection"
	.size	.L__unnamed_20, 10

	.type	.L__unnamed_21,@object
	.section	.rodata..L__unnamed_21,"a",@progbits
.L__unnamed_21:
	.ascii	"lay_wayland::connection"
	.size	.L__unnamed_21, 23

	.type	.L__unnamed_22,@object
	.section	.rodata..L__unnamed_22,"a",@progbits
.L__unnamed_22:
	.ascii	"lay-wayland/src/connection.rs"
	.size	.L__unnamed_22, 29

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_2:
	.quad	.L__unnamed_20
	.asciz	"\n\000\000\000\000\000\000"
	.quad	.L__unnamed_21
	.asciz	"\027\000\000\000\000\000\000"
	.quad	.L__unnamed_22
	.asciz	"\035\000\000\000\000\000\000"
	.size	.L__unnamed_2, 48

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"XDG_RUNTIME_DIR"
	.size	.L__unnamed_3, 15

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.ascii	"WAYLAND_DISPLAY"
	.size	.L__unnamed_4, 15

	.type	.L__unnamed_23,@object
	.section	.rodata..L__unnamed_23,"a",@progbits
.L__unnamed_23:
	.ascii	"Attempting to connect with XDG_RUNTIME + WAYLAND_DISPLAY: "
	.size	.L__unnamed_23, 58

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_5:
	.quad	.L__unnamed_23
	.asciz	":\000\000\000\000\000\000"
	.size	.L__unnamed_5, 16

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.ascii	"wayland-0"
	.size	.L__unnamed_6, 9

	.type	.L__unnamed_24,@object
	.section	.rodata..L__unnamed_24,"a",@progbits
.L__unnamed_24:
	.ascii	"Attempting to connect with XDG_RUNTIME_DIR + wayland-0: "
	.size	.L__unnamed_24, 56

	.type	.L__unnamed_7,@object
	.section	.data.rel.ro..L__unnamed_7,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_7:
	.quad	.L__unnamed_24
	.asciz	"8\000\000\000\000\000\000"
	.size	.L__unnamed_7, 16

	.type	.L__unnamed_25,@object
	.section	.rodata..L__unnamed_25,"a",@progbits
.L__unnamed_25:
	.ascii	"Unable to connect to wayland socket"
	.size	.L__unnamed_25, 35

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_8:
	.quad	.L__unnamed_25
	.asciz	"#\000\000\000\000\000\000"
	.size	.L__unnamed_8, 16

	.type	.L__unnamed_9,@object
	.section	.rodata..L__unnamed_9,"a",@progbits
.L__unnamed_9:
	.ascii	"WlFixed"
	.size	.L__unnamed_9, 7

	.type	.L__unnamed_10,@object
	.section	.data.rel.ro..L__unnamed_10,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_10:
	.quad	_ZN4core3ptr24drop_in_place$LT$f32$GT$17h2f13d589a9447b91E
	.asciz	"\004\000\000\000\000\000\000\000\004\000\000\000\000\000\000"
	.quad	_ZN4core3fmt5float50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$f32$GT$3fmt17h60268ebe8cf6c341E
	.size	.L__unnamed_10, 32

	.type	.L__unnamed_26,@object
	.section	.rodata..L__unnamed_26,"a",@progbits
.L__unnamed_26:
	.ascii	"lay-wayland/src/utils.rs"
	.size	.L__unnamed_26, 24

	.type	.L__unnamed_11,@object
	.section	.data.rel.ro..L__unnamed_11,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_11:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000F\000\000\000=\000\000"
	.size	.L__unnamed_11, 24

	.type	.L__unnamed_12,@object
	.section	.data.rel.ro..L__unnamed_12,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_12:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000`\000\000\000K\000\000"
	.size	.L__unnamed_12, 24

	.type	.L__unnamed_13,@object
	.section	.data.rel.ro..L__unnamed_13,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_13:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000n\000\000\000M\000\000"
	.size	.L__unnamed_13, 24

	.type	.L__unnamed_14,@object
	.section	.data.rel.ro..L__unnamed_14,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_14:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000\177\000\000\000E\000\000"
	.size	.L__unnamed_14, 24

	.type	.L__unnamed_16,@object
	.section	.data.rel.ro..L__unnamed_16,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_16:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000\206\000\000\000.\000\000"
	.size	.L__unnamed_16, 24

	.type	.L__unnamed_17,@object
	.section	.data.rel.ro..L__unnamed_17,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_17:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000\211\000\000\000A\000\000"
	.size	.L__unnamed_17, 24

	.type	.L__unnamed_15,@object
	.section	.data.rel.ro..L__unnamed_15,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_15:
	.quad	.L__unnamed_26
	.asciz	"\030\000\000\000\000\000\000\000\242\000\000\000$\000\000"
	.size	.L__unnamed_15, 24

	.type	.L__unnamed_27,@object
	.section	.rodata..L__unnamed_27,"a",@progbits
.L__unnamed_27:
	.ascii	"0x"
	.size	.L__unnamed_27, 2

	.type	.L__unnamed_28,@object
	.section	.rodata..L__unnamed_28,"a",@progbits
.L__unnamed_28:
	.ascii	" 0x"
	.size	.L__unnamed_28, 3

	.type	.L__unnamed_18,@object
	.section	.data.rel.ro..L__unnamed_18,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_18:
	.quad	.L__unnamed_27
	.asciz	"\002\000\000\000\000\000\000"
	.quad	.L__unnamed_28
	.asciz	"\003\000\000\000\000\000\000"
	.size	.L__unnamed_18, 32

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.79.0 (129f3b996 2024-06-10)"
	.section	".note.GNU-stack","",@progbits
