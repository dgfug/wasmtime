;;! target = "x86_64"
;;! test = "compile"
;;! flags = " -C cranelift-enable-heap-access-spectre-mitigation=false -O static-memory-forced -O static-memory-guard-size=0 -O dynamic-memory-guard-size=0"

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i32 1)

  (func (export "do_store") (param i32 i32)
    local.get 0
    local.get 1
    i32.store offset=0)

  (func (export "do_load") (param i32) (result i32)
    local.get 0
    i32.load offset=0))

;; wasm[0]::function[0]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movl    %edx, %r9d
;;       cmpq    0x1a(%rip), %r9
;;       ja      0x21
;;   14: movq    0x60(%rdi), %r11
;;       movl    %ecx, (%r11, %r9)
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;   21: ud2
;;   23: addb    %al, (%rax)
;;   25: addb    %al, (%rax)
;;   27: addb    %bh, %ah
;;
;; wasm[0]::function[1]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movl    %edx, %r9d
;;       cmpq    0x1a(%rip), %r9
;;       ja      0x51
;;   44: movq    0x60(%rdi), %r11
;;       movl    (%r11, %r9), %eax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;   51: ud2
;;   53: addb    %al, (%rax)
;;   55: addb    %al, (%rax)
;;   57: addb    %bh, %ah
