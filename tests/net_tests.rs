// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use huanlang::stdlib::net::*;
use huanlang::stdlib::time::*;
use std::thread;

#[test]
fn test_udp_socket() {
    // 测试UDP套接字
    let mut socket1 = UDP套接字::绑定("127.0.0.1", 0).unwrap();
    let addr1 = socket1.本地地址().unwrap();
    
    let mut socket2 = UDP套接字::新建().unwrap();
    
    // 发送数据
    let data = b"Hello UDP";
    socket2.发送到(data, &addr1).unwrap();
    
    // 接收数据
    let mut buffer = [0; 1024];
    let (n, addr2) = socket1.接收从(&mut buffer).unwrap();
    assert_eq!(n, data.len());
    assert_eq!(&buffer[..n], data);
    
    // 关闭套接字
    socket1.关闭().unwrap();
    socket2.关闭().unwrap();
}

#[test]
fn test_http_client() {
    // 测试HTTP客户端
    let client = HTTP客户端::新建();
    
    // 测试GET请求
    let response = client.获取("https://httpbin.org/get").unwrap();
    assert_eq!(response.状态码, 200);
    
    // 测试响应内容
    let content = response.内容作为字符串().unwrap();
    assert!(content.contains("origin"));
}

#[test]
fn test_http_server() {
    // 启动HTTP服务器
    let mut server = HTTP服务器::新建();
    
    // 添加路由
    server.路由("/test", |_req| {
        HTTP服务器响应::成功("Hello from HTTP server!")
    });
    
    // 在后台线程启动服务器
    thread::spawn(move || {
        server.监听(8080).unwrap();
    });
    
    // 等待服务器启动
    睡眠(&持续时间::从秒(1.0));
    
    // 测试客户端请求
    let client = HTTP客户端::新建();
    let response = client.获取("http://localhost:8080/test").unwrap();
    assert_eq!(response.状态码, 200);
    
    let content = response.内容作为字符串().unwrap();
    assert_eq!(content, "Hello from HTTP server!");
}
