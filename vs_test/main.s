	.text
	.file	"main.c"
	.globl	sys_key_count                   ; -- Begin function sys_key_count
	.type	sys_key_count,@function
sys_key_count:                          ; @sys_key_count
; %bb.0:                                ; %entry
	;APP
	mov R0, SYS_KEY_COUNT
	;NO_APP
	RET
.Lfunc_end0:
	.size	sys_key_count, .Lfunc_end0-sys_key_count
                                        ; -- End function
	.globl	press_and_release_gk            ; -- Begin function press_and_release_gk
	.type	press_and_release_gk,@function
press_and_release_gk:                   ; @press_and_release_gk
; %bb.0:                                ; %entry
	;APP
	PRESS_GK_VAL R0
	;NO_APP
	;APP
	SLEEP_U16 1000
	;NO_APP
	;APP
	RELEASE_GK_VAL R0
	;NO_APP
	;APP
	SLEEP_U16 1000
	;NO_APP
	RET
.Lfunc_end1:
	.size	press_and_release_gk, .Lfunc_end1-press_and_release_gk
                                        ; -- End function
	.globl	sayo_malloc                     ; -- Begin function sayo_malloc
	.type	sayo_malloc,@function
sayo_malloc:                            ; @sayo_malloc
; %bb.0:                                ; %entry
	;APP
	MALLOC R0
	;NO_APP
	RET
.Lfunc_end2:
	.size	sayo_malloc, .Lfunc_end2-sayo_malloc
                                        ; -- End function
	.globl	sayo_free                       ; -- Begin function sayo_free
	.type	sayo_free,@function
sayo_free:                              ; @sayo_free
; %bb.0:                                ; %entry
	;APP
	FREE R0
	;NO_APP
	RET
.Lfunc_end3:
	.size	sayo_free, .Lfunc_end3-sayo_free
                                        ; -- End function
	.globl	get_gl_size                     ; -- Begin function get_gl_size
	.type	get_gl_size,@function
get_gl_size:                            ; @get_gl_size
; %bb.0:                                ; %entry
	;APP
	mov R0, GL_SIZE
	;NO_APP
	RET
.Lfunc_end4:
	.size	get_gl_size, .Lfunc_end4-get_gl_size
                                        ; -- End function
	.globl	set_gl                          ; -- Begin function set_gl
	.type	set_gl,@function
set_gl:                                 ; @set_gl
; %bb.0:                                ; %entry
	MOV8 R2, 1
	JG R0, R2, .LBB5_4
; %bb.1:                                ; %entry
	JZ R0, .LBB5_7
; %bb.2:                                ; %entry
	XOR_R R0, R0, R2
	JZ R0, .LBB5_3
	JMP .LBB5_9
.LBB5_3:                                ; %if.then2
	;APP
	mov GL_1, R1
	;NO_APP
	RET
.LBB5_4:                                ; %entry
	MOV8 R2, 2
	XOR_R R2, R0, R2
	JZ R2, .LBB5_8
; %bb.5:                                ; %entry
	MOV8 R2, 3
	XOR_R R0, R0, R2
	JZ R0, .LBB5_6
	JMP .LBB5_9
.LBB5_6:                                ; %if.then8
	;APP
	mov GL_3, R1
	;NO_APP
	RET
.LBB5_7:                                ; %if.then
	;APP
	mov GL_0, R1
	;NO_APP
	RET
.LBB5_8:                                ; %if.then5
	;APP
	mov GL_2, R1
	;NO_APP
.LBB5_9:                                ; %if.end11
	RET
.Lfunc_end5:
	.size	set_gl, .Lfunc_end5-set_gl
                                        ; -- End function
	.globl	get_gl                          ; -- Begin function get_gl
	.type	get_gl,@function
get_gl:                                 ; @get_gl
; %bb.0:                                ; %entry
	MOV R1, R0
	MOV8 R0, 0
	MOV8 R2, 1
	JG R1, R2, .LBB6_4
; %bb.1:                                ; %entry
	JZ R1, .LBB6_7
; %bb.2:                                ; %entry
	XOR_R R1, R1, R2
	JZ R1, .LBB6_3
	JMP .LBB6_9
.LBB6_3:                                ; %if.then2
	;APP
	mov R0, GL_1
	;NO_APP
	RET
.LBB6_4:                                ; %entry
	MOV8 R2, 2
	XOR_R R2, R1, R2
	JZ R2, .LBB6_8
; %bb.5:                                ; %entry
	MOV8 R2, 3
	XOR_R R1, R1, R2
	JZ R1, .LBB6_6
	JMP .LBB6_9
.LBB6_6:                                ; %if.then8
	;APP
	mov R0, GL_3
	;NO_APP
	RET
.LBB6_7:                                ; %if.then
	;APP
	mov R0, GL_0
	;NO_APP
	RET
.LBB6_8:                                ; %if.then5
	;APP
	mov R0, GL_2
	;NO_APP
.LBB6_9:                                ; %if.end11
	RET
.Lfunc_end6:
	.size	get_gl, .Lfunc_end6-get_gl
                                        ; -- End function
	.globl	get_arg                         ; -- Begin function get_arg
	.type	get_arg,@function
get_arg:                                ; @get_arg
; %bb.0:                                ; %entry
	MOV R1, R0
	MOV8 R0, 0
	MOV8 R2, 1
	JG R1, R2, .LBB7_4
; %bb.1:                                ; %entry
	JZ R1, .LBB7_7
; %bb.2:                                ; %entry
	XOR_R R1, R1, R2
	JZ R1, .LBB7_3
	JMP .LBB7_9
.LBB7_3:                                ; %if.then2
	;APP
	mov R0, V1
	;NO_APP
	RET
.LBB7_4:                                ; %entry
	MOV8 R2, 2
	XOR_R R2, R1, R2
	JZ R2, .LBB7_8
; %bb.5:                                ; %entry
	MOV8 R2, 3
	XOR_R R1, R1, R2
	JZ R1, .LBB7_6
	JMP .LBB7_9
.LBB7_6:                                ; %if.then8
	;APP
	mov R0, V3
	;NO_APP
	RET
.LBB7_7:                                ; %if.then
	;APP
	mov R0, V0
	;NO_APP
	RET
.LBB7_8:                                ; %if.then5
	;APP
	mov R0, V2
	;NO_APP
.LBB7_9:                                ; %if.end11
	RET
.Lfunc_end7:
	.size	get_arg, .Lfunc_end7-get_arg
                                        ; -- End function
	.globl	wait_if_press                   ; -- Begin function wait_if_press
	.type	wait_if_press,@function
wait_if_press:                          ; @wait_if_press
; %bb.0:                                ; %entry
	;APP
	WAIT_IF_PRESS
	;NO_APP
	RET
.Lfunc_end8:
	.size	wait_if_press, .Lfunc_end8-wait_if_press
                                        ; -- End function
	.globl	wait_if_release                 ; -- Begin function wait_if_release
	.type	wait_if_release,@function
wait_if_release:                        ; @wait_if_release
; %bb.0:                                ; %entry
	;APP
	WAIT_IF_RELEASE
	;NO_APP
	RET
.Lfunc_end9:
	.size	wait_if_release, .Lfunc_end9-wait_if_release
                                        ; -- End function
	.globl	end_script                      ; -- Begin function end_script
	.type	end_script,@function
end_script:                             ; @end_script
; %bb.0:                                ; %entry
	;APP
	EXIT
	;NO_APP
	RET
.Lfunc_end10:
	.size	end_script, .Lfunc_end10-end_script
                                        ; -- End function
	.globl	print_number                    ; -- Begin function print_number
	.type	print_number,@function
print_number:                           ; @print_number
; %bb.0:                                ; %entry
	SUB8 R15, 32
	MOV R1, R15
	ADD8 R1, 28
	mov *R1_32b, R4

	PRINT_REG R4
	SLEEP_U16 1000
	PRINT_REG *R1_32b
	SLEEP_U16 1000 


	MOV R2, R15
	ADD8 R2, 24
	mov *R2_32b, R5
	MOV R3, R15
	ADD8 R3, 20
	mov *R3_32b, R6
	MOV R1, R15
	ADD8 R1, 16
	mov *R1_32b, R7
	MOV R1, R15
	ADD8 R1, 12
	mov *R1_32b, R8
	MOV8 R4, 12
	MOV8 R1, 10
	MOV8 R2, 39
	MOV8 R3, 0
	MOV R5, R15
	MOV8 R6, 9
	JMP .LBB11_1
.LBB11_3:                               ; %while.body
                                        ;   in Loop: Header=BB11_1 Depth=1
	DEC R4
	ADD_R R0, R4, R5
	mov *R0, R8
	DIV_R R0, R7, R1 ;udiv
	JG R7, R6, .LBB11_1
	JMP .LBB11_4
.LBB11_1:                               ; %while.body
                                        ; =>This Inner Loop Header: Depth=1
	MOV R7, R0
	MOD_R R0, R7, R1 ;srem
	MOV R8, R2
	CJNE R0, R3, .LBB11_2
	JMP .LBB11_3
.LBB11_2:                               ;   in Loop: Header=BB11_1 Depth=1
	ADD8 R0, 29
	MOV R8, R0
	JMP .LBB11_3
.LBB11_4:
	MOV8 R6, 12
.LBB11_5:                               ; %for.body
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R0, R5, R4
	mov R0, *R0
	CALL press_and_release_gk
	INC R4
	XOR_R R0, R4, R6
	JZ R0, .LBB11_6
	JMP .LBB11_5
.LBB11_6:                               ; %for.cond.cleanup
	MOV R0, R15
	ADD8 R0, 12
	mov R8, *R0_32b
	MOV R1, R15
	ADD8 R1, 16
	mov R7, *R1_32b
	MOV R2, R15
	ADD8 R2, 20
	mov R6, *R2_32b
	MOV R3, R15
	ADD8 R3, 24
	mov R5, *R3_32b
	MOV R1, R15
	ADD8 R1, 28
	mov R4, *R1_32b
	PRINT_REG R4
	SLEEP_U16 1000
	PRINT_REG *R1_32b
	SLEEP_U16 1000 
	ADD8 R15, 32
	RET
.Lfunc_end11:
	.size	print_number, .Lfunc_end11-print_number
                                        ; -- End function
	.globl	init                            ; -- Begin function init
	.type	init,@function
init:                                   ; @init
; %bb.0:                                ; %entry
	SUB8 R15, 8
	MOV R0, R15
	ADD8 R0, 4
	mov *R0_32b, R4
	MOV R1, R15
	mov *R1_32b, R5
	MOV8 R4, 64
	MOV R0, R4
	CALL sayo_malloc
	MOV R1, R0
	MOV8 R3, 0
.LBB12_1:                               ; %for.body
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R2, R1, R3
	MOV32 R0, level
	ADD R0, SCRIPT_ADDR
	ADD_R R0, R3, R0
	mov R5, *R0
	MOV R0, R2
	mov *R0, R5
	INC R3
	XOR_R R0, R3, R4
	JZ R0, .LBB12_2
	JMP .LBB12_1
.LBB12_2:                               ; %for.cond.cleanup
	MOV8 R0, 0
	CALL print_number
	MOV8 R4, 40
	MOV R0, R4
	CALL press_and_release_gk
	MOV8 R0, 1
	CALL print_number
	MOV R0, R4
	CALL press_and_release_gk
	MOV8 R0, 2
	CALL print_number
	MOV R0, R4
	CALL press_and_release_gk
	MOV8 R0, 3
	CALL print_number
	MOV R0, R4
	CALL press_and_release_gk
	MOV8 R0, 4
	CALL print_number
	MOV R0, R4
	CALL press_and_release_gk
	MOV8 R0, 5
	CALL print_number
	MOV R0, R4
	CALL press_and_release_gk
	mov R5, *R0_32b
	MOV R1, R15
	ADD8 R1, 4
	mov R4, *R1_32b
	ADD8 R15, 8
	RET
.Lfunc_end12:
	.size	init, .Lfunc_end12-init
                                        ; -- End function
	.globl	main                            ; -- Begin function main
	.type	main,@function
main:                                   ; @main
; %bb.0:                                ; %entry
	MOV32 R15, 2048
	MALLOC R15
	MOV32 R14, 2048
	ADD_R R15, R15, R14
	MOV8 R0, 0
	CALL get_arg
	JNZ R0, .LBB13_2
; %bb.1:                                ; %sw.bb
	CALL init
.LBB13_2:                               ; %sw.epilog
	CALL end_script
	MOV8 R0, 0
	RET
.Lfunc_end13:
	.size	main, .Lfunc_end13-main
                                        ; -- End function
	.type	level,@object                   ; @level
	.data
	.globl	level
level:
	.ascii	"    ,,,, ,7 ,,,, ,,   ,, \033\037,, ,, ,,!, ,, ,,   ,,    ,,,,,,,,,,,,"
	.size	level, 64

	.ident	"clang version 17.0.6 (ssh://git@code.misakanet.cn:10777/svr2kos2/llvm_sayo_backend.git 599914bb46922f92f8520e0e28db156fd60bbae1)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
