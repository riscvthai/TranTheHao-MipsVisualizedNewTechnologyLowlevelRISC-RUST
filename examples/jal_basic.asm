.data
message: .asciiz "MIPS JAL example\n"

.text
.globl main

main:
    jal print_message

    li $v0, 10
    syscall

print_message:
    li $v0, 4
    la $a0, message
    syscall

    jr $ra
