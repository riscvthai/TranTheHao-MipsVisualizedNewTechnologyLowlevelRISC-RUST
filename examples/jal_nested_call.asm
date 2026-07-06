# Trần Thế Hảo — MIPS JAL nested call example
# Purpose: show $ra overwrite hazard and restoration.

main:
  li $t0, 1
  jal function_a
  li $v0, 10
  syscall

function_a:
  move $t2, $ra
  addi $t0, $t0, 10
  jal function_b
  move $ra, $t2
  jr $ra

function_b:
  addi $t0, $t0, 100
  jr $ra
