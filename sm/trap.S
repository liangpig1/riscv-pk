enclave_trap_table:
  .word bad_trap
  .word pmp_trap
  .word illegal_insn_trap
  .word bad_trap
  .word misaligned_load_trap
  .word pmp_trap
  .word misaligned_store_trap
  .word pmp_trap
  .word bad_trap
  .word mcall_trap_enclave // 9 --> ECALL from S-mode
  .word bad_trap
  .word bad_trap
  .word bad_trap


  .option norvc
  .section .text.init,"ax",@progbits
  .globl reset_vector
  .globl switch_vector_enclave
  .globl switch_vector_host


