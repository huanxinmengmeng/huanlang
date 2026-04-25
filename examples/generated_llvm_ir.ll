; ModuleID = 'huanlang_module'
source_filename = "huanlang_source.hl"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

declare i32 @puts(i8*) nounwind
declare i32 @printf(i8*, ...) nounwind
declare i32 @scanf(i8*, ...) nounwind
declare i8* @malloc(i64) nounwind
declare void @free(i8*) nounwind
declare i64 @strlen(i8*) nounwind
declare i8* @strcpy(i8*, i8*) nounwind
declare i8* @strcat(i8*, i8*) nounwind
declare i32 @strcmp(i8*, i8*) nounwind

define i32 @add(i32 %arg0, i32 %arg1) {
bb0:
  %0 = alloca i32, align 4
  store i32 %arg0, i32* %0, align 4
  %1 = alloca i32, align 4
  store i32 %arg1, i32* %1, align 4
  %2 = load i32, i32* %0, align 4
  %3 = load i32, i32* %1, align 4
  %4 = add i32 %2, %3
  ret i32 %4
}

define i32 @main() {
bb0:
  %0 = alloca i32, align 4
  store i32 42, i32* %0, align 4
  %1 = load i32, i32* %0, align 4
  %2 = add i32 %1, 10
  %3 = alloca i32, align 4
  store i32 %2, i32* %3, align 4
  %4 = load i32, i32* %0, align 4
  %5 = load i32, i32* %3, align 4
  %6 = call i32 @add(i32 %4, i32 %5)
  %7 = alloca i32, align 4
  store i32 %6, i32* %7, align 4
  %8 = load i32, i32* %7, align 4
  ret i32 %8
}

