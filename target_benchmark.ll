; ModuleID = 'vif'
source_filename = "vif"

@format_str = private unnamed_addr constant [8 x i8] c"%d %d \0A\00", align 1

declare i64 @printf(ptr, ...)

define ptr @main() {
entry:
  %fib = call i64 @fib(i64 40)
  %printf = call i64 (ptr, ...) @printf(ptr @format_str, i64 0, i64 %fib)
  %0 = alloca i64, align 8
  store i64 %printf, ptr %0, align 4
  %1 = alloca i64, align 8
  store i64 1, ptr %0, align 4
  ret ptr %1
}

define i64 @fib(i64 %0) {
entry:
  %1 = icmp slt i64 %0, 2
  br i1 %1, label %then, label %end

end:                                              ; preds = %entry, %then
  %2 = sub i64 %0, 2
  %fib = call i64 @fib(i64 %2)
  %3 = sub i64 %0, 1
  %fib1 = call i64 @fib(i64 %3)
  %4 = add i64 %fib, %fib1
  ret i64 %4

then:                                             ; preds = %entry
  ret i64 %0
  br label %end
}
