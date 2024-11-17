; ModuleID = 'vif'
source_filename = "vif"

@hello_world = private unnamed_addr constant [15 x i8] c"Hello, World!\0A\00", align 1

declare ptr @puts(ptr %0)

define i32 @main() {
entry:
  %call_puts = call ptr @puts(ptr @hello_world)
  ret i32 0
}
