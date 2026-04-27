// 幻语 MLIR 方言测试文件
// 测试huan方言的基本操作和降级流程

module {
  // 测试基本算术操作
  func.func @test_arith() -> i64 {
    // 测试常量
    %c42 = arith.constant 42 : i64
    %c100 = arith.constant 100 : i64
    
    // 测试加法 (huan.add -> arith.addi)
    %add = arith.addi %c42, %c100 : i64
    
    return %add : i64
  }
  
  // 测试控制流
  func.func @test_control_flow(%arg0: i1) -> i64 {
    %c0 = arith.constant 0 : i64
    %c1 = arith.constant 1 : i64
    
    // 测试scf.if
    %result = scf.if %arg0 -> (i64) {
      scf.yield %c1 : i64
    } else {
      scf.yield %c0 : i64
    }
    
    return %result : i64
  }
  
  // 测试函数调用
  func.func @test_call() -> i64 {
    %c5 = arith.constant 5 : i64
    %result = call @add_two(%c5) : (i64) -> i64
    return %result : i64
  }
  
  func.func @add_two(%arg0: i64) -> i64 {
    %c2 = arith.constant 2 : i64
    %result = arith.addi %arg0, %c2 : i64
    return %result : i64
  }
}
