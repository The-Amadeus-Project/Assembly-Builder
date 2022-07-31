pub const PRINT_STR_LITERAL: &str = r#"
    PRINT_STR_LITERAL:
		push 			rdi
		mov 			rax, rdi
        mov   			rbx, 0

    .loop:
          inc   rax
          inc   rbx
          mov   cl, [rax]
          cmp  cl, 0
          jne   .loop

          mov   rax, 1
          mov   rdi, 1
          pop   rsi
          mov   rdx, rbx
          syscall
          ret
"#;