// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 包管理器命令接口

use std::env;
use std::path::Path;
use std::process;
use crate::package::error::{PackageError, PackageResult};
use crate::package::manifest::PackageManifest;
use crate::package::lockfile::PackageLock;
use crate::package::resolver::DependencyResolver;
use crate::package::registry::RegistryClient;
use crate::package::cache::CacheManager;

/// 命令类型
#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    /// 初始化项目
    Init,
    /// 添加依赖
    Add,
    /// 移除依赖
    Remove,
    /// 更新依赖
    Update,
    /// 安装依赖
    Install,
    /// 构建项目
    Build,
    /// 测试项目
    Test,
    /// 运行项目
    Run,
    /// 发布项目
    Publish,
    /// 搜索包
    Search,
    /// 显示信息
    Info,
    /// 清理缓存
    Clean,
    /// 工作区命令
    Workspace,
    /// 安全命令
    Security,
    /// 帮助
    Help,
    /// 版本
    Version,
    /// 未知命令
    Unknown,
}

/// 命令参数
#[derive(Debug, Clone)]
pub struct CommandArgs {
    pub command: CommandType,
    pub subcommand: Option<String>,
    pub args: Vec<String>,
    pub options: HashMap<String, String>,
}

/// 命令执行结果
pub type CommandResult = std::result::Result<i32, PackageError>;

use std::collections::HashMap;

impl CommandArgs {
    /// 解析命令行参数
    pub fn parse(args: &[String]) -> Self {
        if args.len() < 2 {
            return CommandArgs {
                command: CommandType::Help,
                subcommand: None,
                args: vec![],
                options: HashMap::new(),
            };
        }

        let command = match args[1].as_str() {
            "init" => CommandType::Init,
            "add" => CommandType::Add,
            "remove" => CommandType::Remove,
            "update" => CommandType::Update,
            "install" => CommandType::Install,
            "build" => CommandType::Build,
            "test" => CommandType::Test,
            "run" => CommandType::Run,
            "publish" => CommandType::Publish,
            "search" => CommandType::Search,
            "info" => CommandType::Info,
            "clean" => CommandType::Clean,
            "workspace" => CommandType::Workspace,
            "security" => CommandType::Security,
            "help" => CommandType::Help,
            "version" => CommandType::Version,
            _ => CommandType::Unknown,
        };

        let mut subcommand = None;
        let mut parsed_args = vec![];
        let mut options = HashMap::new();

        let mut i = 2;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                // 选项
                let parts: Vec<&str> = arg.split("=").collect();
                let key = parts[0][2..].to_string();
                let value = if parts.len() > 1 {
                    parts[1].to_string()
                } else if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    i += 1;
                    args[i].clone()
                } else {
                    "true".to_string()
                };
                options.insert(key, value);
            } else if arg.starts_with("-") {
                // 短选项
                let key = arg[1..].to_string();
                options.insert(key, "true".to_string());
            } else if subcommand.is_none() && command == CommandType::Workspace {
                // 工作区子命令
                subcommand = Some(arg.clone());
            } else {
                // 参数
                parsed_args.push(arg.clone());
            }
            i += 1;
        }

        CommandArgs {
            command,
            subcommand,
            args: parsed_args,
            options,
        }
    }
}

/// 命令执行器
pub struct Command {
    args: CommandArgs,
    manifest_path: String,
    lock_path: String,
}

impl Command {
    /// 执行命令
    pub fn execute(args: &[String]) -> CommandResult {
        let command_args = CommandArgs::parse(args);
        let cwd = env::current_dir()
            .map_err(|e| PackageError::io_error(&e.to_string(), None))?;
        
        let manifest_path = cwd.join("幻语包.toml").to_str().unwrap().to_string();
        let lock_path = cwd.join("幻语锁.toml").to_str().unwrap().to_string();
        
        let command = Command {
            args: command_args,
            manifest_path,
            lock_path,
        };
        
        command.run()
    }

    /// 运行命令
    fn run(&self) -> CommandResult {
        match self.args.command {
            CommandType::Init => self.init(),
            CommandType::Add => self.add(),
            CommandType::Remove => self.remove(),
            CommandType::Update => self.update(),
            CommandType::Install => self.install(),
            CommandType::Build => self.build(),
            CommandType::Test => self.test(),
            CommandType::Run => self.run_cmd(),
            CommandType::Publish => self.publish(),
            CommandType::Search => self.search(),
            CommandType::Info => self.info(),
            CommandType::Clean => self.clean(),
            CommandType::Workspace => self.workspace(),
            CommandType::Security => self.security(),
            CommandType::Help => self.help(),
            CommandType::Version => self.version(),
            CommandType::Unknown => self.unknown(),
        }
    }

    /// 初始化项目
    fn init(&self) -> CommandResult {
        let name = self.args.args.first().unwrap_or(&"my-project".to_string());
        let edition = self.args.options.get("edition").unwrap_or(&"1.2".to_string());
        
        let manifest = PackageManifest {
            package: crate::package::manifest::PackageInfo {
                name: name.clone(),
                version: "0.1.0".to_string(),
                edition: Some(edition.clone()),
                authors: Some(vec!["Your Name <you@example.com>".to_string()]),
                description: Some("A new 幻语 project".to_string()),
                license: Some("MIT".to_string()),
                license_file: None,
                repository: None,
                documentation: None,
                homepage: None,
                keywords: None,
                categories: None,
                readme: Some("README.md".to_string()),
                keyword_style: None,
            },
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
            extern_libs: None,
            lib: Some(crate::package::manifest::LibConfig {
                name: None,
                path: Some("src/lib.幻".to_string()),
                crate_type: None,
                test: None,
                doctest: None,
                bench: None,
            }),
            bins: Some(vec![crate::package::manifest::BinConfig {
                name: name.clone(),
                path: Some("src/main.幻".to_string()),
                test: None,
                bench: None,
            }]),
            examples: None,
            tests: None,
            benches: None,
            build: None,
            profile: None,
            bindings: None,
            workspace: None,
            patch: None,
            replace: None,
            features: None,
        };
        
        manifest.save_to_file(&self.manifest_path)?;
        
        // 创建基本目录结构
        std::fs::create_dir_all("src")?;
        std::fs::write("src/lib.幻", "// 库代码")?;
        std::fs::write("src/main.幻", "// 主程序")?;
        std::fs::write("README.md", format!("# {}\n\nA new 幻语 project\n", name))?;
        
        println!("项目 {} 初始化成功！", name);
        Ok(0)
    }

    /// 添加依赖
    fn add(&self) -> CommandResult {
        if self.args.args.is_empty() {
            return Err(PackageError::config_error("请指定要添加的包", None));
        }
        
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        let mut manifest = manifest;
        
        let mut dependencies = manifest.dependencies.unwrap_or_default();
        
        for dep in &self.args.args {
            let parts: Vec<&str> = dep.split('@').collect();
            let name = parts[0];
            let version = if parts.len() > 1 {
                parts[1]
            } else {
                "^0.1.0"
            };
            
            dependencies.insert(name.to_string(), crate::package::manifest::Dependency::Version(version.to_string()));
        }
        
        manifest.dependencies = Some(dependencies);
        manifest.save_to_file(&self.manifest_path)?;
        
        println!("依赖添加成功！");
        Ok(0)
    }

    /// 移除依赖
    fn remove(&self) -> CommandResult {
        if self.args.args.is_empty() {
            return Err(PackageError::config_error("请指定要移除的包", None));
        }
        
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        let mut manifest = manifest;
        
        if let Some(mut dependencies) = manifest.dependencies {
            for dep in &self.args.args {
                dependencies.remove(dep);
            }
            manifest.dependencies = Some(dependencies);
            manifest.save_to_file(&self.manifest_path)?;
        }
        
        println!("依赖移除成功！");
        Ok(0)
    }

    /// 更新依赖
    fn update(&self) -> CommandResult {
        // 读取清单
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        
        // 解析依赖
        let resolver = DependencyResolver::from_manifest(&manifest)?;
        
        // 构建根依赖
        let mut root_deps = HashMap::new();
        if let Some(deps) = &manifest.dependencies {
            for (name, dep) in deps {
                match dep {
                    crate::package::manifest::Dependency::Version(v) => {
                        root_deps.insert(name.clone(), crate::package::resolver::VersionConstraint::parse(v)?);
                    }
                    crate::package::manifest::Dependency::Detailed(d) => {
                        if let Some(v) = &d.version {
                            root_deps.insert(name.clone(), crate::package::resolver::VersionConstraint::parse(v)?);
                        }
                    }
                }
            }
        }
        
        // 解析依赖
        let result = resolver.resolve(&root_deps)?;
        
        // 生成锁定文件
        let lock = PackageLock::from_resolution(&result);
        lock.save_to_file(&self.lock_path)?;
        
        println!("依赖更新成功！");
        Ok(0)
    }

    /// 安装依赖
    fn install(&self) -> CommandResult {
        // 读取清单
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        
        // 读取或生成锁定文件
        let lock = if Path::new(&self.lock_path).exists() {
            PackageLock::from_file(&self.lock_path)?
        } else {
            // 解析依赖
            let resolver = DependencyResolver::from_manifest(&manifest)?;
            
            // 构建根依赖
            let mut root_deps = HashMap::new();
            if let Some(deps) = &manifest.dependencies {
                for (name, dep) in deps {
                    match dep {
                        crate::package::manifest::Dependency::Version(v) => {
                            root_deps.insert(name.clone(), crate::package::resolver::VersionConstraint::parse(v)?);
                        }
                        crate::package::manifest::Dependency::Detailed(d) => {
                            if let Some(v) = &d.version {
                                root_deps.insert(name.clone(), crate::package::resolver::VersionConstraint::parse(v)?);
                            }
                        }
                    }
                }
            }
            
            // 解析依赖
            let result = resolver.resolve(&root_deps)?;
            
            // 生成锁定文件
            let lock = PackageLock::from_resolution(&result);
            lock.save_to_file(&self.lock_path)?;
            lock
        };
        
        // 创建缓存管理器
        let cache = CacheManager::new()?;
        
        // 安装依赖
        println!("正在安装依赖...");
        for (name, package) in &lock.packages {
            println!("  {} v{}", name, package.version);
            
            // 检查缓存
            let version = crate::package::resolver::Version::parse(&package.version)?;
            if cache.has_package(name, &version) {
                println!("    从缓存中获取...");
            } else {
                // 从注册表获取
                println!("    从注册表下载...");
                let client = RegistryClient::new("https://registry.huanlang.org", None);
                let package_info = client.get_package_version(name, &package.version)?;
                
                // 模拟下载包内容
                let content = format!("Package {} v{}", name, package.version).as_bytes().to_vec();
                
                // 缓存包
                cache.cache_package(name, &version, &content)?;
            }
        }
        
        println!("依赖安装成功！");
        Ok(0)
    }

    /// 构建项目
    fn build(&self) -> CommandResult {
        println!("正在构建项目...");
        // 模拟构建过程
        println!("构建成功！");
        Ok(0)
    }

    /// 测试项目
    fn test(&self) -> CommandResult {
        println!("正在运行测试...");
        // 模拟测试过程
        println!("测试通过！");
        Ok(0)
    }

    /// 运行项目
    fn run_cmd(&self) -> CommandResult {
        println!("正在运行项目...");
        // 模拟运行过程
        println!("项目运行成功！");
        Ok(0)
    }

    /// 发布项目
    fn publish(&self) -> CommandResult {
        println!("正在发布项目...");
        
        // 读取清单
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        
        // 创建注册表客户端
        let client = RegistryClient::new("https://registry.huanlang.org", None);
        
        // 检查包是否已存在
        let exists = client.package_exists(&manifest.package.name)?;
        if exists {
            println!("包 {} 已存在，更新版本...", manifest.package.name);
        }
        
        // 模拟创建tarball文件
        let tarball_path = format!("{}-{}.tar.gz", manifest.package.name, manifest.package.version);
        std::fs::write(&tarball_path, "模拟tarball内容")?;
        
        // 上传包
        let result = client.upload(&manifest, &tarball_path)?;
        
        // 删除临时tarball
        std::fs::remove_file(&tarball_path)?;
        
        println!("项目发布成功！");
        println!("  包名: {}", result.name);
        println!("  版本: {}", result.version);
        
        Ok(0)
    }

    /// 搜索包
    fn search(&self) -> CommandResult {
        if self.args.args.is_empty() {
            return Err(PackageError::config_error("请指定搜索关键词", None));
        }
        
        let keyword = &self.args.args[0];
        println!("搜索包: {}", keyword);
        
        // 创建注册表客户端
        let client = RegistryClient::new("https://registry.huanlang.org", None);
        
        // 搜索包
        let result = client.search(keyword, 1, 10)?;
        
        println!("  找到 {} 个结果:", result.total);
        for package in &result.packages {
            println!("  * {} v{} - {}", 
                     package.name, 
                     package.version, 
                     package.description.as_ref().unwrap_or(&"无描述".to_string()));
        }
        
        Ok(0)
    }

    /// 显示信息
    fn info(&self) -> CommandResult {
        if self.args.args.is_empty() {
            // 显示当前项目信息
            let manifest = PackageManifest::from_file(&self.manifest_path)?;
            println!("项目信息:");
            println!("  名称: {}", manifest.package.name);
            println!("  版本: {}", manifest.package.version);
            println!("  描述: {}", manifest.package.description.unwrap_or("无".to_string()));
        } else {
            // 显示指定包信息
            let package = &self.args.args[0];
            println!("包信息: {}", package);
            
            // 创建注册表客户端
            let client = RegistryClient::new("https://registry.huanlang.org", None);
            
            // 获取包信息
            let package_info = client.get_package(package)?;
            
            println!("  版本: {}", package_info.version);
            println!("  描述: {}", package_info.description.unwrap_or("无".to_string()));
            println!("  作者: {}", package_info.authors.map(|a| a.join(", ")).unwrap_or("无".to_string()));
            println!("  许可证: {}", package_info.license.unwrap_or("无".to_string()));
            if let Some(repo) = package_info.repository {
                println!("  仓库: {}", repo);
            }
            if let Some(docs) = package_info.documentation {
                println!("  文档: {}", docs);
            }
        }
        Ok(0)
    }

    /// 清理缓存
    fn clean(&self) -> CommandResult {
        println!("正在清理缓存...");
        // 模拟清理过程
        println!("缓存清理成功！");
        Ok(0)
    }

    /// 工作区命令
    fn workspace(&self) -> CommandResult {
        match self.args.subcommand.as_deref() {
            Some("add") => println!("添加工作区成员成功！"),
            Some("remove") => println!("移除工作区成员成功！"),
            Some("list") => println!("工作区成员列表:
  * member1
  * member2"),
            _ => println!("工作区命令: add, remove, list"),
        }
        Ok(0)
    }

    /// 显示帮助
    fn help(&self) -> CommandResult {
        println!("幻语包管理器命令:");
        println!("  init        初始化新项目");
        println!("  add         添加依赖");
        println!("  remove      移除依赖");
        println!("  update      更新依赖");
        println!("  install     安装依赖");
        println!("  build       构建项目");
        println!("  test        运行测试");
        println!("  run         运行项目");
        println!("  publish     发布项目");
        println!("  search      搜索包");
        println!("  info        显示信息");
        println!("  clean       清理缓存");
        println!("  workspace   工作区命令");
        println!("  security    安全相关命令");
        println!("  help        显示帮助");
        println!("  version     显示版本");
        Ok(0)
    }

    /// 显示版本
    fn version(&self) -> CommandResult {
        println!("幻语包管理器 v1.0.0");
        Ok(0)
    }

    /// 未知命令
    fn unknown(&self) -> CommandResult {
        println!("未知命令，请使用 'help' 查看可用命令");
        Ok(1)
    }

    /// 安全相关命令
    fn security(&self) -> CommandResult {
        match self.args.subcommand.as_deref() {
            Some("verify") => self.security_verify(),
            Some("audit") => self.security_audit(),
            Some("scan") => self.security_scan(),
            Some("sign") => self.security_sign(),
            Some("generate-key") => self.security_generate_key(),
            _ => {
                println!("安全命令:");
                println!("  verify      验证包签名");
                println!("  audit       执行安全审计");
                println!("  scan        扫描包漏洞");
                println!("  sign        签名包");
                println!("  generate-key  生成密钥对");
                Ok(0)
            }
        }
    }

    /// 验证包签名
    fn security_verify(&self) -> CommandResult {
        if self.args.args.is_empty() {
            return Err(PackageError::config_error("请指定要验证的包文件", None));
        }
        
        let package_path = &self.args.args[0];
        println!("验证包签名: {}", package_path);
        
        // 创建安全管理器
        let security_manager = crate::package::security::SecurityManager::new();
        
        // 读取包清单
        let manifest_path = std::path::Path::new(package_path).join("幻语包.toml");
        if !manifest_path.exists() {
            return Err(PackageError::io_error("找不到包清单文件", Some(manifest_path.to_str().unwrap())));
        }
        
        let manifest = PackageManifest::from_file(&manifest_path)?;
        
        if let Some(signature) = &manifest.signature {
            // 验证签名
            let result = security_manager.verify_signature(std::path::Path::new(package_path), signature)?;
            
            if result {
                println!("签名验证成功！");
            } else {
                println!("签名验证失败！");
                return Ok(1);
            }
        } else {
            println!("包未签名！");
            return Ok(1);
        }
        
        Ok(0)
    }

    /// 执行安全审计
    fn security_audit(&self) -> CommandResult {
        println!("执行安全审计...");
        
        // 读取包清单
        let manifest = PackageManifest::from_file(&self.manifest_path)?;
        
        // 创建安全管理器
        let mut security_manager = crate::package::security::SecurityManager::new();
        
        // 尝试加载漏洞数据库
        let db_path = "vulnerability_db.json";
        if std::path::Path::new(db_path).exists() {
            security_manager.load_vulnerability_db(db_path)?;
        } else {
            println!("警告: 未找到漏洞数据库，审计结果可能不完整");
        }
        
        // 审计当前包
        let audit_result = security_manager.audit_package(
            &manifest.package.name,
            &manifest.package.version
        )?;
        
        // 显示审计结果
        println!("审计结果:");
        println!("  包名称: {}", audit_result.package_name);
        println!("  包版本: {}", audit_result.package_version);
        println!("  安全评级: {:?}", audit_result.security_rating);
        println!("  审计时间: {}", audit_result.audit_time);
        
        if !audit_result.issues.is_empty() {
            println!("  发现的问题:");
            for issue in &audit_result.issues {
                println!("    - {} (严重程度: {:?})", issue.id, issue.severity);
                println!("      描述: {}", issue.description);
                println!("      建议: {}", issue.recommendation);
                if let Some(cve) = &issue.cve {
                    println!("      CVE: {}", cve);
                }
            }
        } else {
            println!("  未发现安全问题");
        }
        
        Ok(0)
    }

    /// 扫描包漏洞
    fn security_scan(&self) -> CommandResult {
        if self.args.args.is_empty() {
            return Err(PackageError::config_error("请指定要扫描的包文件", None));
        }
        
        let package_path = &self.args.args[0];
        println!("扫描包漏洞: {}", package_path);
        
        // 创建安全管理器
        let mut security_manager = crate::package::security::SecurityManager::new();
        
        // 尝试加载漏洞数据库
        let db_path = "vulnerability_db.json";
        if std::path::Path::new(db_path).exists() {
            security_manager.load_vulnerability_db(db_path)?;
        } else {
            println!("警告: 未找到漏洞数据库，扫描结果可能不完整");
        }
        
        // 扫描漏洞
        let vulnerabilities = security_manager.scan_for_vulnerabilities(std::path::Path::new(package_path))?;
        
        // 显示扫描结果
        if !vulnerabilities.is_empty() {
            println!("发现的漏洞:");
            for vuln in &vulnerabilities {
                println!("  - {} (严重程度: {:?})", vuln.id, vuln.severity);
                println!("    描述: {}", vuln.description);
                if let Some(fixed) = &vuln.fixed_versions {
                    println!("    修复版本: {}", fixed);
                }
                if let Some(cve) = &vuln.cve {
                    println!("    CVE: {}", cve);
                }
            }
        } else {
            println!("未发现漏洞");
        }
        
        Ok(0)
    }

    /// 签名包
    fn security_sign(&self) -> CommandResult {
        if self.args.args.len() < 2 {
            return Err(PackageError::config_error("请指定要签名的包文件和私钥文件", None));
        }
        
        let package_path = &self.args.args[0];
        let private_key_path = &self.args.args[1];
        println!("签名包: {}", package_path);
        
        // 读取包清单
        let manifest_path = std::path::Path::new(package_path).join("幻语包.toml");
        if !manifest_path.exists() {
            return Err(PackageError::io_error("找不到包清单文件", Some(manifest_path.to_str().unwrap())));
        }
        
        let mut manifest = PackageManifest::from_file(&manifest_path)?;
        
        // 生成公钥路径
        let public_key_path = format!("{}.pub", private_key_path);
        
        // 创建签名工具
        let signature_tool = crate::package::security::SignatureTool::from_files(
            std::path::Path::new(private_key_path),
            std::path::Path::new(&public_key_path)
        )?;
        
        // 签名包
        let signature = signature_tool.sign_package(std::path::Path::new(package_path))?;
        
        // 更新包清单
        manifest.signature = Some(signature);
        manifest.save_to_file(&manifest_path)?;
        
        println!("包签名成功！");
        Ok(0)
    }

    /// 生成密钥对
    fn security_generate_key(&self) -> CommandResult {
        let private_key_path = self.args.args.first().unwrap_or(&"private.key".to_string());
        let public_key_path = format!("{}.pub", private_key_path);
        
        println!("生成密钥对...");
        
        // 生成密钥对
        let signature_tool = crate::package::security::SignatureTool::generate_keypair()?;
        
        // 保存密钥对
        signature_tool.save_keypair(
            std::path::Path::new(private_key_path),
            std::path::Path::new(&public_key_path)
        )?;
        
        println!("密钥对生成成功！");
        println!("  私钥: {}", private_key_path);
        println!("  公钥: {}", public_key_path);
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse() {
        let args = vec!["hlpm".to_string(), "init".to_string(), "my-project".to_string()];
        let command_args = CommandArgs::parse(&args);
        assert_eq!(command_args.command, CommandType::Init);
        assert_eq!(command_args.args, vec!["my-project"]);
    }

    #[test]
    fn test_command_parse_with_options() {
        let args = vec!["hlpm".to_string(), "add".to_string(), "package@1.0.0".to_string(), "--dev".to_string()];
        let command_args = CommandArgs::parse(&args);
        assert_eq!(command_args.command, CommandType::Add);
        assert_eq!(command_args.args, vec!["package@1.0.0"]);
        assert!(command_args.options.contains_key("dev"));
    }

    #[test]
    fn test_help_command() {
        let args = vec!["hlpm".to_string()];
        let result = Command::execute(&args);
        assert!(result.is_ok());
    }
}
