; Flux Language - Generated LLVM IR
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(i8*, ...)
declare i8* @malloc(i64)
declare void @free(i8*)

@.str_num = private unnamed_addr constant [6 x i8] c"%f\0A\00"
@.str_str = private unnamed_addr constant [4 x i8] c"%s\0A\00"
@.str_bool_true = private unnamed_addr constant [6 x i8] c"true\0A\00"
@.str_bool_false = private unnamed_addr constant [7 x i8] c"false\0A\00"

%temporal_entry = type { double, i8* }
%temporal_var = type { i32, %temporal_entry* }

define void @flux_main() {
entry:
define double @double(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t1 = load double, double* %x
  %t2 = fadd double 0.0, 2
  %t3 = fmul double %t1, %t2
  ret double %%t3
  ret double 0.0
}

define double @add_ten(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t4 = load double, double* %x
  %t5 = fadd double 0.0, 10
  %t6 = fadd double %t4, %t5
  ret double %%t6
  ret double 0.0
}

define double @square(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t7 = load double, double* %x
  %t8 = load double, double* %x
  %t9 = fmul double %t7, %t8
  ret double %%t9
  ret double 0.0
}

define double @to_string(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t10 = load double, double* %x
  %t11 = fadd double 0, %t10
  ret double %%t11
  ret double 0.0
}

define double @is_even(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t12 = load double, double* %x
  %t13 = fadd double 0.0, 2
  %t14 = fadd double %t12, %t13
  %t15 = fadd double 0.0, 0
  %t16_cmp = fcmp oeq double %t14, %t15
  %t16 = uitofp i1 %t16_cmp to double
  ret double %%t16
  ret double 0.0
}

  %t17 = fadd double 0.0, 5
  %value = alloca double
  store double %%t17, double* %value
  %result = alloca double
  store double %0, double* %result
  %complex_result = alloca double
  store double %0, double* %complex_result
define double @add_prefix(double) {
entry:
  %text = alloca double
  store double %0, double* %text
  %t18 = load double, double* %text
  %t19 = fadd double 0, %t18
  ret double %%t19
  ret double 0.0
}

define double @add_suffix(double) {
entry:
  %text = alloca double
  store double %0, double* %text
  %t20 = load double, double* %text
  %t21 = fadd double %t20, 0
  ret double %%t21
  ret double 0.0
}

define double @to_uppercase(double) {
entry:
  %text = alloca double
  store double %0, double* %text
  %t22 = load double, double* %text
  ret double %%t22
  ret double 0.0
}

  %text_pipeline = alloca double
  store double %0, double* %text_pipeline
define double @multiply_by(double) {
entry:
  %multiplier = alloca double
  store double %0, double* %multiplier
define double @inner(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t23 = load double, double* %x
  %t24 = load double, double* %multiplier
  %t25 = fmul double %t23, %t24
  ret double %%t25
  ret double 0.0
}

  %t26 = load double, double* %inner
  ret double %%t26
  ret double 0.0
}

define double @divide_by(double) {
entry:
  %divisor = alloca double
  store double %0, double* %divisor
define double @inner(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t27 = load double, double* %x
  %t28 = load double, double* %divisor
  %t29 = fdiv double %t27, %t28
  ret double %%t29
  ret double 0.0
}

  %t30 = load double, double* %inner
  ret double %%t30
  ret double 0.0
}

define double @round_to_int(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t31 = load double, double* %x
  ret double %%t31
  ret double 0.0
}

  %t32 = fadd double 0.0, 3
  %t33 = call double @multiply_by(%t32)
  %multiply_by_3 = alloca double
  store double %%t33, double* %multiply_by_3
  %t34 = fadd double 0.0, 2
  %t35 = call double @divide_by(%t34)
  %divide_by_2 = alloca double
  store double %%t35, double* %divide_by_2
  %number_result = alloca double
  store double %0, double* %number_result
define double @filter_positive(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t36 = load double, double* %x
  %t37 = fadd double 0.0, 0
  %t38 = fadd double %t36, %t37
  %t39 = fcmp une double %%t38, 0.0
  br i1 %t39, label %L1, label %L2
L1:
  %t40 = load double, double* %x
  ret double %%t40
  br label %L3
L2:
  %t41 = fadd double 0.0, 0
  ret double %%t41
  br label %L3
L3:
  ret double 0.0
}

define double @clamp_max(double) {
entry:
  %max_val = alloca double
  store double %0, double* %max_val
define double @inner(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t42 = load double, double* %x
  %t43 = load double, double* %max_val
  %t44 = fadd double %t42, %t43
  %t45 = fcmp une double %%t44, 0.0
  br i1 %t45, label %L4, label %L5
L4:
  %t46 = load double, double* %max_val
  ret double %%t46
  br label %L6
L5:
  %t47 = load double, double* %x
  ret double %%t47
  br label %L6
L6:
  ret double 0.0
}

  %t48 = load double, double* %inner
  ret double %%t48
  ret double 0.0
}

define double @normalize(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t49 = load double, double* %x
  %t50 = fadd double 0.0, 100
  %t51 = fdiv double %t49, %t50
  ret double %%t51
  ret double 0.0
}

  %data_point = alloca double
  store double %0, double* %data_point
  %processed = alloca double
  store double %0, double* %processed
define double @process_if_even(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t52 = load double, double* %x
  %t53 = call double @is_even(%t52)
  %t54 = fcmp une double %%t53, 0.0
  br i1 %t54, label %L7, label %L8
L7:
  ret double %0
  br label %L9
L8:
  ret double %0
  br label %L9
L9:
  ret double 0.0
}

  %t55 = fadd double 0.0, 4
  %even_number = alloca double
  store double %%t55, double* %even_number
  %t56 = fadd double 0.0, 5
  %odd_number = alloca double
  store double %%t56, double* %odd_number
  %t57 = load double, double* %even_number
  %t58 = call double @process_if_even(%t57)
  %even_result = alloca double
  store double %%t58, double* %even_result
  %t59 = load double, double* %odd_number
  %t60 = call double @process_if_even(%t59)
  %odd_result = alloca double
  store double %%t60, double* %odd_result
define double @validate_positive(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t61 = load double, double* %x
  %t62 = fadd double 0.0, 0
  %t63 = fadd double %t61, %t62
  %t64 = fcmp une double %%t63, 0.0
  br i1 %t64, label %L10, label %L12
L10:
  %t65 = load double, double* %x
  %t66 = fmul double %t65, 0
  ret double %%t66
  br label %L12
L12:
  %t67 = load double, double* %x
  ret double %%t67
  ret double 0.0
}

define double @safe_divide_by_ten(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  %t68 = load double, double* %x
  %t69 = fadd double 0.0, 0
  %t70_cmp = fcmp oeq double %t68, %t69
  %t70 = uitofp i1 %t70_cmp to double
  %t71 = fcmp une double %%t70, 0.0
  br i1 %t71, label %L13, label %L15
L13:
  %t72 = fadd double 0.0, 0
  ret double %%t72
  br label %L15
L15:
  %t73 = load double, double* %x
  %t74 = fadd double 0.0, 10
  %t75 = fdiv double %t73, %t74
  ret double %%t75
  ret double 0.0
}

  %test_values = alloca double
  store double %0, double* %test_values
define double @process_value(double) {
entry:
  %val = alloca double
  store double %0, double* %val
  ret double %0
  ret double 0.0
}

  %processed_values = alloca double
  store double %0, double* %processed_values
define double @create_math_pipeline() {
entry:
define double @pipeline(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  ret double %0
  ret double 0.0
}

  %t76 = load double, double* %pipeline
  ret double %%t76
  ret double 0.0
}

define double @create_string_pipeline() {
entry:
define double @pipeline(double) {
entry:
  %x = alloca double
  store double %0, double* %x
  ret double %0
  ret double 0.0
}

  %t77 = load double, double* %pipeline
  ret double %%t77
  ret double 0.0
}

  %t78 = call double @create_math_pipeline()
  %math_pipe = alloca double
  store double %%t78, double* %math_pipe
  %t79 = call double @create_string_pipeline()
  %string_pipe = alloca double
  store double %%t79, double* %string_pipe
  %t80 = fadd double 0.0, 3
  %t81 = call double @math_pipe(%t80)
  %math_result = alloca double
  store double %%t81, double* %math_result
  %t82 = fadd double 0.0, 42
  %t83 = call double @string_pipe(%t82)
  %string_result = alloca double
  store double %%t83, double* %string_result
  ret void
}


define i32 @main() {
entry:
  call void @flux_main()
  ret i32 0
}
