// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 幻语测试框架完整示例

use huanlang::test::*;

/// 示例模块 - 数学函数
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
    
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("除数不能为零");
        }
        a / b
    }
    
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }
}

/// 示例模块 - 列表操作
mod list_utils {
    pub fn append<T: Clone>(list: &mut Vec<T>, item: T) {
        list.push(item);
    }
    
    pub fn remove_index<T>(list: &mut Vec<T>, index: usize) -> T {
        list.remove(index)
    }
    
    pub fn length<T>(list: &[T]) -> usize {
        list.len()
    }
}

/// 测试注册 - 幻语测试格式
fn register_tests() {
    let module = TestModuleBuilder::new("数学函数测试".to_string())
        .submodule(
            TestModuleBuilder::new("基本运算".to_string())
                .test("加法测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
                .test("减法测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
                .test("除法测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
        )
        .submodule(
            TestModuleBuilder::new("三角函数".to_string())
                .test("正弦测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
        );
    
    for entry in module.build() {
        register_test(entry);
    }
}

/// 测试函数 - 加法
fn test_addition() -> huanlang::test::TestErrorResult<()> {
    use math::*;
    
    assert_eq_impl(add(2, 3), 5, file!().to_string(), line!(), column!())?;
    assert_eq_impl(add(-1, 1), 0, file!().to_string(), line!(), column!())?;
    assert_eq_impl(add(0, 0), 0, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 测试函数 - 减法
fn test_subtraction() -> huanlang::test::TestErrorResult<()> {
    use math::*;
    
    assert_eq_impl(subtract(10, 3), 7, file!().to_string(), line!(), column!())?;
    assert_eq_impl(subtract(5, 10), -5, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 测试函数 - 除法
fn test_division() -> huanlang::test::TestErrorResult<()> {
    use math::*;
    
    assert_eq_impl(divide(10, 2), 5, file!().to_string(), line!(), column!())?;
    assert_eq_impl(divide(0, 5), 0, file!().to_string(), line!(), column!())?;
    
    // 测试除零错误
    assert_panics_with_impl(
        || { divide(10, 0); },
        "除数不能为零",
        file!().to_string(),
        line!(),
        column!(),
    )?;
    
    Ok(())
}

/// 测试函数 - 正弦
fn test_sine() -> huanlang::test::TestErrorResult<()> {
    use math::*;
    
    let pi = std::f64::consts::PI;
    assert_approx_impl(sin(0.0), 0.0, 0.0001, file!().to_string(), line!(), column!())?;
    assert_approx_impl(sin(pi / 2.0), 1.0, 0.0001, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 列表操作测试模块
fn register_list_tests() {
    let module = TestModuleBuilder::new("列表操作测试".to_string())
        .test("长度测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
        .test("追加测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()))
        .test("删除测试".to_string(), SourceLocation::new(file!().to_string(), line!(), column!()));
    
    for entry in module.build() {
        register_test(entry);
    }
}

/// 测试函数 - 长度
fn test_length() -> huanlang::test::TestErrorResult<()> {
    use list_utils::*;
    
    let list = vec![1, 2, 3, 4, 5];
    assert_eq_impl(length(&list), 5, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 测试函数 - 追加
fn test_append() -> huanlang::test::TestErrorResult<()> {
    use list_utils::*;
    
    let mut list = vec![1, 2, 3];
    append(&mut list, 4);
    assert_eq_impl(length(&list), 4, file!().to_string(), line!(), column!())?;
    assert_list_contains_impl(&list, &4, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 测试函数 - 删除
fn test_remove() -> huanlang::test::TestErrorResult<()> {
    use list_utils::*;
    
    let mut list = vec![1, 2, 3, 4, 5];
    let removed = remove_index(&mut list, 2);
    assert_eq_impl(removed, 3, file!().to_string(), line!(), column!())?;
    assert_eq_impl(length(&list), 4, file!().to_string(), line!(), column!())?;
    
    Ok(())
}

/// 基准测试
fn run_benchmarks() {
    println!("运行基准测试...");
    
    let mut runner = BenchmarkRunner::new();
    
    // 简单运算基准
    runner.run("简单加法".to_string(), || {
        let _ = 1 + 2;
    });
    
    // 列表操作基准
    runner.run("列表追加".to_string(), || {
        let mut list = Vec::with_capacity(1000);
        for i in 0..1000 {
            list.push(i);
        }
        std::hint::black_box(list);
    });
    
    // 列表排序基准
    runner.run("列表排序".to_string(), || {
        let mut list = vec![5, 3, 1, 4, 2];
        list.sort();
        std::hint::black_box(list);
    });
    
    runner.print_results();
}

/// 工具函数使用示例
fn utility_examples() {
    println!("工具函数示例:");
    
    // 临时目录
    // 注意：这里使用标准库的临时目录功能
    let temp_dir = std::env::temp_dir();
    println!("  临时目录: {:?}", temp_dir);
    
    // 测量时间
    let start = std::time::Instant::now();
    // 执行一些操作
    for _ in 0..1000000 {
        let _ = 1 + 1;
    }
    let duration = start.elapsed();
    println!("  循环耗时: {}ms", duration.as_millis());
}

/// 主函数
fn main() {
    println!("幻语测试框架完整示例");
    println!("=====================");
    println!();
    
    // 注册测试
    register_tests();
    register_list_tests();
    
    // 运行单元测试
    let config = TestConfig::default();
    let mut runner = TestRunner::new(config);
    
    let _ = runner.discover(&[]);
    let _ = runner.run();
    
    println!();
    println!("基准测试:");
    println!("=========");
    println!();
    
    run_benchmarks();
    
    println!();
    println!("工具函数示例:");
    println!("=============");
    println!();
    
    utility_examples();
}
