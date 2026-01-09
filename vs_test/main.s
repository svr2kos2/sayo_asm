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
	.globl	print_reg                       ; -- Begin function print_reg
	.type	print_reg,@function
print_reg:                              ; @print_reg
; %bb.0:                                ; %entry
	;APP
	PRINT_REG R0
	;NO_APP
	RET
.Lfunc_end1:
	.size	print_reg, .Lfunc_end1-print_reg
                                        ; -- End function
	.globl	press_and_release_gk            ; -- Begin function press_and_release_gk
	.type	press_and_release_gk,@function
press_and_release_gk:                   ; @press_and_release_gk
; %bb.0:                                ; %entry
	AND8 R0, 255
	;APP
	PRESS_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	RET
.Lfunc_end2:
	.size	press_and_release_gk, .Lfunc_end2-press_and_release_gk
                                        ; -- End function
	.globl	sayo_malloc                     ; -- Begin function sayo_malloc
	.type	sayo_malloc,@function
sayo_malloc:                            ; @sayo_malloc
; %bb.0:                                ; %entry
	;APP
	MALLOC R0
	;NO_APP
	RET
.Lfunc_end3:
	.size	sayo_malloc, .Lfunc_end3-sayo_malloc
                                        ; -- End function
	.globl	sayo_free                       ; -- Begin function sayo_free
	.type	sayo_free,@function
sayo_free:                              ; @sayo_free
; %bb.0:                                ; %entry
	;APP
	FREE R0
	;NO_APP
	RET
.Lfunc_end4:
	.size	sayo_free, .Lfunc_end4-sayo_free
                                        ; -- End function
	.globl	gl_size                         ; -- Begin function gl_size
	.type	gl_size,@function
gl_size:                                ; @gl_size
; %bb.0:                                ; %entry
	;APP
	mov R0, GL_SIZE
	;NO_APP
	RET
.Lfunc_end5:
	.size	gl_size, .Lfunc_end5-gl_size
                                        ; -- End function
	.globl	set_gl                          ; -- Begin function set_gl
	.type	set_gl,@function
set_gl:                                 ; @set_gl
; %bb.0:                                ; %entry
	MOV8 R2, 1
	JG R0, R2, .LBB6_4
; %bb.1:                                ; %entry
	JZ R0, .LBB6_7
; %bb.2:                                ; %entry
	XOR_R R0, R0, R2
	JZ R0, .LBB6_3
	JMP .LBB6_9
.LBB6_3:                                ; %if.then2
	;APP
	mov GL_1, R1
	;NO_APP
	RET
.LBB6_4:                                ; %entry
	MOV8 R2, 2
	XOR_R R2, R0, R2
	JZ R2, .LBB6_8
; %bb.5:                                ; %entry
	MOV8 R2, 3
	XOR_R R0, R0, R2
	JZ R0, .LBB6_6
	JMP .LBB6_9
.LBB6_6:                                ; %if.then8
	;APP
	mov GL_3, R1
	;NO_APP
	RET
.LBB6_7:                                ; %if.then
	;APP
	mov GL_0, R1
	;NO_APP
	RET
.LBB6_8:                                ; %if.then5
	;APP
	mov GL_2, R1
	;NO_APP
.LBB6_9:                                ; %if.end11
	RET
.Lfunc_end6:
	.size	set_gl, .Lfunc_end6-set_gl
                                        ; -- End function
	.globl	get_gl                          ; -- Begin function get_gl
	.type	get_gl,@function
get_gl:                                 ; @get_gl
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
	mov R0, GL_1
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
	mov R0, GL_3
	;NO_APP
	RET
.LBB7_7:                                ; %if.then
	;APP
	mov R0, GL_0
	;NO_APP
	RET
.LBB7_8:                                ; %if.then5
	;APP
	mov R0, GL_2
	;NO_APP
.LBB7_9:                                ; %if.end11
	RET
.Lfunc_end7:
	.size	get_gl, .Lfunc_end7-get_gl
                                        ; -- End function
	.globl	get_arg                         ; -- Begin function get_arg
	.type	get_arg,@function
get_arg:                                ; @get_arg
; %bb.0:                                ; %entry
	MOV R1, R0
	MOV8 R0, 0
	MOV8 R2, 1
	JG R1, R2, .LBB8_4
; %bb.1:                                ; %entry
	JZ R1, .LBB8_7
; %bb.2:                                ; %entry
	XOR_R R1, R1, R2
	JZ R1, .LBB8_3
	JMP .LBB8_9
.LBB8_3:                                ; %if.then2
	;APP
	mov R0, V1
	;NO_APP
	RET
.LBB8_4:                                ; %entry
	MOV8 R2, 2
	XOR_R R2, R1, R2
	JZ R2, .LBB8_8
; %bb.5:                                ; %entry
	MOV8 R2, 3
	XOR_R R1, R1, R2
	JZ R1, .LBB8_6
	JMP .LBB8_9
.LBB8_6:                                ; %if.then8
	;APP
	mov R0, V3
	;NO_APP
	RET
.LBB8_7:                                ; %if.then
	;APP
	mov R0, V0
	;NO_APP
	RET
.LBB8_8:                                ; %if.then5
	;APP
	mov R0, V2
	;NO_APP
.LBB8_9:                                ; %if.end11
	RET
.Lfunc_end8:
	.size	get_arg, .Lfunc_end8-get_arg
                                        ; -- End function
	.globl	wait_if_press                   ; -- Begin function wait_if_press
	.type	wait_if_press,@function
wait_if_press:                          ; @wait_if_press
; %bb.0:                                ; %entry
	;APP
	WAIT_IF_PRESS
	;NO_APP
	RET
.Lfunc_end9:
	.size	wait_if_press, .Lfunc_end9-wait_if_press
                                        ; -- End function
	.globl	wait_if_release                 ; -- Begin function wait_if_release
	.type	wait_if_release,@function
wait_if_release:                        ; @wait_if_release
; %bb.0:                                ; %entry
	;APP
	WAIT_IF_RELEASE
	;NO_APP
	RET
.Lfunc_end10:
	.size	wait_if_release, .Lfunc_end10-wait_if_release
                                        ; -- End function
	.globl	find_player                     ; -- Begin function find_player
	.type	find_player,@function
find_player:                            ; @find_player
; %bb.0:                                ; %entry
	SUB8 R15, 8
	MOV R1, R15
	ADD8 R1, 4
	mov *R1_32b, R4
	MOV R2, R15
	mov *R2_32b, R5
	MOV R2, R0
	MOV8 R3, 0
	MOV8 R4, 31
	MOV8 R5, 64
	MOV R1, R3
.LBB11_1:                               ; %for.body
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R0, R2, R1
	mov R0, *R0
	OR8 R0, 4
	XOR_R R0, R0, R4
	JZ R0, .LBB11_4
; %bb.2:                                ; %for.inc
                                        ;   in Loop: Header=BB11_1 Depth=1
	INC R1
	XOR_R R0, R1, R5
	JZ R0, .LBB11_3
	JMP .LBB11_1
.LBB11_3:
	MOV R1, R3
.LBB11_4:                               ; %cleanup
	MOV R0, R1
	mov R5, *R1_32b
	MOV R2, R15
	ADD8 R2, 4
	mov R4, *R2_32b
	ADD8 R15, 8
	RET
.Lfunc_end11:
	.size	find_player, .Lfunc_end11-find_player
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
	MOV8 R3, 64
	MOV R2, R3
	;APP
	MALLOC R2
	;NO_APP
	MOV8 R4, 0
.LBB12_1:                               ; %for.body
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R1, R2, R4
	MOV32 R0, level
	ADD R0, SCRIPT_ADDR
	ADD_R R0, R4, R0
	mov R5, *R0
	MOV R0, R1
	mov *R0, R5
	INC R4
	XOR_R R0, R4, R3
	JZ R0, .LBB12_2
	JMP .LBB12_1
.LBB12_2:                               ; %for.cond.cleanup
	;APP
	mov GL_0, R2
	;NO_APP
	;APP
	WAIT_IF_RELEASE
	;NO_APP
	;APP
	WAIT_IF_PRESS
	;NO_APP
	;APP
	WAIT_IF_RELEASE
	;NO_APP
	mov R5, *R0_32b
	MOV R1, R15
	ADD8 R1, 4
	mov R4, *R1_32b
	ADD8 R15, 8
	RET
.Lfunc_end12:
	.size	init, .Lfunc_end12-init
                                        ; -- End function
	.globl	print_map                       ; -- Begin function print_map
	.type	print_map,@function
print_map:                              ; @print_map
; %bb.0:                                ; %entry
	SUB8 R15, 8
	MOV R1, R15
	ADD8 R1, 4
	mov *R1_32b, R4
	MOV R2, R15
	mov *R2_32b, R5
	MOV R1, R0
	MOV8 R2, 0
	;APP
	PRESS_GK 224
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	PRESS_GK 4
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 4
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 224
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	PRESS_GK 42
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 42
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	MOV8 R3, 8
	MOV R4, R2
.LBB13_1:                               ; %for.cond1.preheader
                                        ; =>This Loop Header: Depth=1
                                        ;     Child Loop BB13_2 Depth 2
	MOV R5, R2
.LBB13_2:                               ; %for.body4
                                        ;   Parent Loop BB13_1 Depth=1
                                        ; =>  This Inner Loop Header: Depth=2
	ADD_R R0, R1, R5
	mov R0, *R0
	;APP
	PRESS_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	INC R5
	XOR_R R0, R5, R3
	JZ R0, .LBB13_3
	JMP .LBB13_2
.LBB13_3:                               ; %for.cond.cleanup3
                                        ;   in Loop: Header=BB13_1 Depth=1
	ADD8 R1, 8
	INC R4
	XOR_R R0, R4, R3
	JZ R0, .LBB13_4
	JMP .LBB13_1
.LBB13_4:                               ; %for.cond.cleanup
	mov R5, *R0_32b
	MOV R1, R15
	ADD8 R1, 4
	mov R4, *R1_32b
	ADD8 R15, 8
	RET
.Lfunc_end13:
	.size	print_map, .Lfunc_end13-print_map
                                        ; -- End function
	.globl	try_move                        ; -- Begin function try_move
	.type	try_move,@function
try_move:                               ; @try_move
; %bb.0:                                ; %entry
	SUB8 R15, 20
	MOV R2, R15
	ADD8 R2, 16
	mov *R2_32b, R4
	MOV R3, R15
	ADD8 R3, 12
	mov *R3_32b, R5
	MOV R1, R15
	ADD8 R1, 8
	mov *R1_32b, R6
	MOV R1, R15
	ADD8 R1, 4
	mov *R1_32b, R7
	MOV R1, R15
	mov *R1_32b, R8
	MOV R2, R0
	MOV8 R6, 0
	;APP
	mov R3, GL_0
	;NO_APP
	MOV8 R4, 31
	MOV8 R7, 64
	MOV R5, R6
.LBB14_1:                               ; %for.body.i
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R0, R3, R5
	mov R0, *R0
	OR8 R0, 4
	XOR_R R0, R0, R4
	JZ R0, .LBB14_4
; %bb.2:                                ; %for.inc.i
                                        ;   in Loop: Header=BB14_1 Depth=1
	INC R5
	XOR_R R0, R5, R7
	JZ R0, .LBB14_3
	JMP .LBB14_1
.LBB14_3:
	MOV R5, R6
.LBB14_4:                               ; %find_player.exit
	MOV8SX R0, -1
	JG R2, R0, .LBB14_6
; %bb.5:                                ; %find_player.exit
	MOV8 R6, 8
	JB R5, R6, .LBB14_26
.LBB14_6:                               ; %lor.lhs.false
	MOV8 R6, 1
	JL R2, R6, .LBB14_8
; %bb.7:                                ; %lor.lhs.false
	MOV R7, R5
	AND32 R7, 2147483640
	MOV8 R8, 56
	XOR_R R7, R7, R8
	JZ R7, .LBB14_26
.LBB14_8:                               ; %lor.lhs.false10
	MOV R7, R5
	AND8 R7, 7
	JG R1, R0, .LBB14_10
; %bb.9:                                ; %lor.lhs.false10
	JZ R7, .LBB14_26
.LBB14_10:                              ; %lor.lhs.false17
	JL R1, R6, .LBB14_12
; %bb.11:                               ; %lor.lhs.false17
	MOV8 R0, 7
	XOR_R R0, R7, R0
	JZ R0, .LBB14_26
.LBB14_12:                              ; %if.end
	SHL8 R2, 3
	ADD_R R6, R2, R1
	ADD_R R7, R6, R5
	ADD_R R1, R3, R7
	MOV R0, R1
	mov R0, *R0
	MOV8 R5, 33
	XOR_R R8, R0, R5
	MOV8 R2, 27
	JZ R8, .LBB14_15
; %bb.13:                               ; %if.end
	MOV8 R8, 32
	XOR_R R8, R0, R8
	JZ R8, .LBB14_26
; %bb.14:                               ; %if.end
	XOR_R R8, R0, R2
	JNZ R8, .LBB14_20
.LBB14_15:                              ; %if.then41
	ADD_R R0, R6, R7
	ADD_R R0, R3, R0
	mov R6, *R0
	MOV R7, R6
	SUB8 R7, 32
	MOV8 R8, 2
	JB R7, R8, .LBB14_26
; %bb.16:                               ; %if.then41
	MOV8 R7, 27
	XOR_R R7, R6, R7
	JZ R7, .LBB14_26
; %bb.17:                               ; %if.then41
	MOV8 R7, 55
	XOR_R R6, R6, R7
	JZ R6, .LBB14_18
	JMP .LBB14_19
.LBB14_18:
	MOV8 R5, 27
.LBB14_19:                              ; %if.end70
	mov *R0, R5
	MOV R0, R1
	mov R0, *R0
.LBB14_20:                              ; %if.end71
	AND8 R0, 255
	MOV8 R5, 55
	CJNE R0, R5, .LBB14_21
	JMP .LBB14_22
.LBB14_21:
	MOV R2, R4
.LBB14_22:                              ; %if.end71
	MOV R0, R1
	mov *R0, R2
	MOV8 R1, 0
	;APP
	PRESS_GK 224
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	PRESS_GK 4
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 4
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 224
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	PRESS_GK 42
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK 42
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	MOV8 R2, 8
	MOV R4, R1
.LBB14_23:                              ; %for.cond1.preheader.i
                                        ; =>This Loop Header: Depth=1
                                        ;     Child Loop BB14_24 Depth 2
	MOV R5, R1
.LBB14_24:                              ; %for.body4.i
                                        ;   Parent Loop BB14_23 Depth=1
                                        ; =>  This Inner Loop Header: Depth=2
	ADD_R R0, R3, R5
	mov R0, *R0
	;APP
	PRESS_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	;APP
	RELEASE_GK_VAL R0
	;NO_APP
	;APP
	SLEEP 10
	;NO_APP
	INC R5
	XOR_R R0, R5, R2
	JZ R0, .LBB14_25
	JMP .LBB14_24
.LBB14_25:                              ; %for.cond.cleanup3.i
                                        ;   in Loop: Header=BB14_23 Depth=1
	ADD8 R3, 8
	INC R4
	XOR_R R0, R4, R2
	JZ R0, .LBB14_26
	JMP .LBB14_23
.LBB14_26:                              ; %cleanup82
	mov R8, *R0_32b
	MOV R1, R15
	ADD8 R1, 4
	mov R7, *R1_32b
	MOV R2, R15
	ADD8 R2, 8
	mov R6, *R2_32b
	MOV R3, R15
	ADD8 R3, 12
	mov R5, *R3_32b
	MOV R1, R15
	ADD8 R1, 16
	mov R4, *R1_32b
	ADD8 R15, 20
	RET
.Lfunc_end14:
	.size	try_move, .Lfunc_end14-try_move
                                        ; -- End function
	.globl	main                            ; -- Begin function main
	.type	main,@function
main:                                   ; @main
; %bb.0:                                ; %entry
	MOV32 R15, 2048
	MALLOC R15
	MOV32 R14, 2048
	ADD_R R15, R15, R14
	SUB8 R15, 8
	MOV R0, R15
	ADD8 R0, 4
	mov *R0_32b, R4
	MOV R1, R15
	mov *R1_32b, R5
	;APP
	mov R0, V0
	;NO_APP
	MOV8 R1, 1
	JG R0, R1, .LBB15_4
	JMP .LBB15_1
.LBB15_4:                               ; %entry
	MOV8 R1, 2
	XOR_R R1, R0, R1
	JZ R1, .LBB15_11
; %bb.5:                                ; %entry
	MOV8 R1, 3
	XOR_R R1, R0, R1
	JZ R1, .LBB15_12
; %bb.6:                                ; %entry
	MOV8 R1, 4
	XOR_R R0, R0, R1
	JZ R0, .LBB15_7
	JMP .LBB15_14
.LBB15_7:                               ; %sw.bb4
	MOV8 R0, 0
	MOV8 R1, 1
	JMP .LBB15_13
.LBB15_1:                               ; %entry
	JZ R0, .LBB15_8
; %bb.2:                                ; %entry
	XOR_R R0, R0, R1
	JZ R0, .LBB15_3
	JMP .LBB15_14
.LBB15_3:                               ; %sw.bb1
	MOV8SX R0, -1
	MOV8 R1, 0
	JMP .LBB15_13
.LBB15_11:                              ; %sw.bb2
	MOV8 R0, 1
	MOV8 R1, 0
	JMP .LBB15_13
.LBB15_12:                              ; %sw.bb3
	MOV8 R0, 0
	MOV8SX R1, -1
.LBB15_13:                              ; %sw.epilog
	CALL try_move
	JMP .LBB15_14
.LBB15_8:                               ; %sw.bb
	MOV8 R3, 64
	MOV R2, R3
	;APP
	MALLOC R2
	;NO_APP
	MOV8 R4, 0
.LBB15_9:                               ; %for.body.i
                                        ; =>This Inner Loop Header: Depth=1
	ADD_R R1, R2, R4
	MOV32 R0, level
	ADD R0, SCRIPT_ADDR
	ADD_R R0, R4, R0
	mov R5, *R0
	MOV R0, R1
	mov *R0, R5
	INC R4
	XOR_R R0, R4, R3
	JNZ R0, .LBB15_9
; %bb.10:                               ; %init.exit
	;APP
	mov GL_0, R2
	;NO_APP
	;APP
	WAIT_IF_RELEASE
	;NO_APP
	;APP
	WAIT_IF_PRESS
	;NO_APP
	;APP
	WAIT_IF_RELEASE
	;NO_APP
.LBB15_14:                              ; %sw.epilog
	MOV8 R0, 0
	mov R5, *R1_32b
	MOV R2, R15
	ADD8 R2, 4
	mov R4, *R2_32b
	ADD8 R15, 8
	RET
.Lfunc_end15:
	.size	main, .Lfunc_end15-main
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
