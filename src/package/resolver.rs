// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 依赖解析模块

use std::collections::{HashMap, HashSet, BTreeSet};
use crate::package::error::{PackageError, PackageResult};
use crate::package::manifest::{PackageManifest, Dependency, DetailedDependency};

/// 版本号
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
    pub build: Option<String>,
}

/// 版本约束
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    Exact(Version),
    Caret(Version),
    Tilde(Version),
    Wildcard(u8), // 0: *, 1: x, 2: x.y
    GreaterThan(Version),
    GreaterThanOrEqual(Version),
    LessThan(Version),
    LessThanOrEqual(Version),
    Range(Version, Version),
    Or(Vec<VersionConstraint>),
}

/// 依赖解析器
pub struct DependencyResolver {
    packages: HashMap<String, BTreeSet<Version>>,
    dependencies: HashMap<String, HashMap<Version, Vec<(String, VersionConstraint)>>>,
}

/// 解析结果
pub struct ResolutionResult {
    pub resolved: HashMap<String, Version>,
    pub conflicts: Vec<(String, VersionConstraint, Version)>,
}

impl Version {
    /// 解析版本字符串
    pub fn parse(version: &str) -> PackageResult<Self> {
        let parts: Vec<&str> = version.split(|c| c == '.' || c == '-' || c == '+').collect();
        
        if parts.len() < 3 {
            return Err(PackageError::version_error("版本格式无效", Some(version)));
        }

        let major = parts[0].parse().map_err(|_| PackageError::version_error("主版本无效", Some(version)))?;
        let minor = parts[1].parse().map_err(|_| PackageError::version_error("次版本无效", Some(version)))?;
        let patch = parts[2].parse().map_err(|_| PackageError::version_error("补丁版本无效", Some(version)))?;

        let mut pre = None;
        let mut build = None;

        for part in parts.iter().skip(3) {
            if part.is_empty() {
                continue;
            }
            if pre.is_none() && !part.chars().all(|c| c.is_digit(10)) {
                pre = Some(part.to_string());
            } else if build.is_none() {
                build = Some(part.to_string());
            }
        }

        Ok(Version {
            major,
            minor,
            patch,
            pre,
            build,
        })
    }

    /// 检查是否兼容
    pub fn is_compatible(&self, other: &Self) -> bool {
        if self.major != other.major {
            return false;
        }
        if self.minor != other.minor {
            return false;
        }
        if self.patch != other.patch {
            return false;
        }
        true
    }

    /// 检查是否满足 caret 约束
    pub fn satisfies_caret(&self, constraint: &Version) -> bool {
        if self.major > constraint.major {
            return false;
        }
        if self.major == constraint.major {
            if self.minor > constraint.minor {
                return false;
            }
            if self.minor == constraint.minor && self.patch < constraint.patch {
                return false;
            }
        }
        true
    }

    /// 检查是否满足 tilde 约束
    pub fn satisfies_tilde(&self, constraint: &Version) -> bool {
        if self.major != constraint.major {
            return false;
        }
        if self.minor > constraint.minor {
            return false;
        }
        if self.minor == constraint.minor && self.patch < constraint.patch {
            return false;
        }
        true
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre) = &self.pre {
            s.push('-');
            s.push_str(pre);
        }
        if let Some(build) = &self.build {
            s.push('+');
            s.push_str(build);
        }
        s
    }
}

impl VersionConstraint {
    /// 解析版本约束
    pub fn parse(constraint: &str) -> PackageResult<Self> {
        let constraint = constraint.trim();
        
        if constraint == "*" {
            return Ok(VersionConstraint::Wildcard(0));
        }
        
        if constraint.ends_with(".*") {
            let parts: Vec<&str> = constraint.split(".").collect();
            if parts.len() == 2 {
                return Ok(VersionConstraint::Wildcard(1));
            } else if parts.len() == 3 {
                return Ok(VersionConstraint::Wildcard(2));
            }
        }
        
        if constraint.starts_with("^") {
            let version = Version::parse(&constraint[1..])?;
            return Ok(VersionConstraint::Caret(version));
        }
        
        if constraint.starts_with("~") {
            let version = Version::parse(&constraint[1..])?;
            return Ok(VersionConstraint::Tilde(version));
        }
        
        if constraint.starts_with(">=") {
            let version = Version::parse(&constraint[2..])?;
            return Ok(VersionConstraint::GreaterThanOrEqual(version));
        }
        
        if constraint.starts_with(">!") {
            let version = Version::parse(&constraint[2..])?;
            return Ok(VersionConstraint::GreaterThan(version));
        }
        
        if constraint.starts_with("<=") {
            let version = Version::parse(&constraint[2..])?;
            return Ok(VersionConstraint::LessThanOrEqual(version));
        }
        
        if constraint.starts_with("<!") {
            let version = Version::parse(&constraint[2..])?;
            return Ok(VersionConstraint::LessThan(version));
        }
        
        if constraint.contains("||") {
            let constraints: Vec<VersionConstraint> = constraint
                .split("||")
                .map(|c| VersionConstraint::parse(c))
                .collect::<PackageResult<Vec<_>>>()?;
            return Ok(VersionConstraint::Or(constraints));
        }
        
        if constraint.contains("..") {
            let parts: Vec<&str> = constraint.split("..").collect();
            if parts.len() == 2 {
                let left = Version::parse(parts[0])?;
                let right = Version::parse(parts[1])?;
                return Ok(VersionConstraint::Range(left, right));
            }
        }
        
        // 默认视为精确版本
        let version = Version::parse(constraint)?;
        Ok(VersionConstraint::Exact(version))
    }

    /// 检查版本是否满足约束
    pub fn satisfies(&self, version: &Version) -> bool {
        match self {
            VersionConstraint::Exact(v) => version == v,
            VersionConstraint::Caret(v) => version.satisfies_caret(v),
            VersionConstraint::Tilde(v) => version.satisfies_tilde(v),
            VersionConstraint::Wildcard(0) => true, // *
            VersionConstraint::Wildcard(1) => true, // x
            VersionConstraint::Wildcard(2) => true, // x.y
            VersionConstraint::GreaterThan(v) => version > v,
            VersionConstraint::GreaterThanOrEqual(v) => version >= v,
            VersionConstraint::LessThan(v) => version < v,
            VersionConstraint::LessThanOrEqual(v) => version <= v,
            VersionConstraint::Range(min, max) => version >= min && version <= max,
            VersionConstraint::Or(constraints) => constraints.iter().any(|c| c.satisfies(version)),
        }
    }
}

impl DependencyResolver {
    /// 创建新的解析器
    pub fn new() -> Self {
        DependencyResolver {
            packages: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// 添加包信息
    pub fn add_package(&mut self, name: &str, version: Version) {
        self.packages
            .entry(name.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(version);
    }

    /// 添加依赖信息
    pub fn add_dependency(&mut self, package: &str, version: Version, dep_name: &str, constraint: VersionConstraint) {
        self.dependencies
            .entry(package.to_string())
            .or_insert_with(HashMap::new)
            .entry(version)
            .or_insert_with(Vec::new)
            .push((dep_name.to_string(), constraint));
    }

    /// 解析依赖
    pub fn resolve(&self, root_deps: &HashMap<String, VersionConstraint>) -> PackageResult<ResolutionResult> {
        // 使用 PubGrub 算法进行依赖解析
        // 这里实现简化版的 PubGrub
        let mut resolved = HashMap::new();
        let mut conflicts = Vec::new();
        let mut queue: Vec<String> = root_deps.keys().cloned().collect();
        let mut visited = HashSet::new();

        while let Some(package) = queue.pop() {
            if visited.contains(&package) {
                continue;
            }
            visited.insert(package.clone());

            let constraint = root_deps.get(&package).unwrap();
            let versions = self.packages.get(&package).ok_or_else(|| {
                PackageError::package_error(&format!("包 {} 不存在", package), Some(&package))
            })?;

            // 选择满足约束的最新版本
            let mut selected_version = None;
            for version in versions.iter().rev() {
                if constraint.satisfies(version) {
                    selected_version = Some(version.clone());
                    break;
                }
            }

            let version = selected_version.ok_or_else(|| {
                PackageError::version_error(
                    &format!("找不到满足约束的版本: {}", constraint),
                    Some(&package)
                )
            })?;

            resolved.insert(package.clone(), version.clone());

            // 添加依赖到队列
            if let Some(deps) = self.dependencies.get(&package) {
                if let Some(package_deps) = deps.get(&version) {
                    for (dep_name, dep_constraint) in package_deps {
                        if !resolved.contains_key(dep_name) {
                            // 检查是否与已解析的版本冲突
                            if let Some(existing_version) = resolved.get(dep_name) {
                                if !dep_constraint.satisfies(existing_version) {
                                    conflicts.push((dep_name.clone(), dep_constraint.clone(), existing_version.clone()));
                                }
                            } else {
                                queue.push(dep_name.clone());
                            }
                        }
                    }
                }
            }
        }

        if !conflicts.is_empty() {
            return Err(PackageError::dependency_error(
                "依赖冲突",
                Some(conflicts.iter().map(|(name, _, _)| (name.clone(), "conflict".to_string())).collect())
            ));
        }

        Ok(ResolutionResult {
            resolved,
            conflicts,
        })
    }

    /// 从清单解析依赖
    pub fn from_manifest(manifest: &PackageManifest) -> PackageResult<Self> {
        let mut resolver = Self::new();
        
        // 添加当前包
        let version = Version::parse(&manifest.package.version)?;
        resolver.add_package(&manifest.package.name, version.clone());
        
        // 添加依赖
        if let Some(deps) = &manifest.dependencies {
            for (name, dep) in deps {
                let constraint = match dep {
                    Dependency::Version(v) => VersionConstraint::parse(v)?,
                    Dependency::Detailed(d) => {
                        if let Some(v) = &d.version {
                            VersionConstraint::parse(v)?
                        } else {
                            VersionConstraint::Wildcard(0)
                        }
                    }
                };
                resolver.add_dependency(&manifest.package.name, version.clone(), name, constraint);
            }
        }
        
        Ok(resolver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);

        let version = Version::parse("1.2.3-beta.1").unwrap();
        assert_eq!(version.pre, Some("beta.1".to_string()));

        let version = Version::parse("1.2.3+build.1").unwrap();
        assert_eq!(version.build, Some("build.1".to_string()));
    }

    #[test]
    fn test_version_constraint() {
        let constraint = VersionConstraint::parse("^1.2.3").unwrap();
        let version = Version::parse("1.2.4").unwrap();
        assert!(constraint.satisfies(&version));

        let version = Version::parse("1.3.0").unwrap();
        assert!(!constraint.satisfies(&version));
    }

    #[test]
    fn test_resolver() {
        let mut resolver = DependencyResolver::new();
        
        // 添加包
        resolver.add_package("a", Version::parse("1.0.0").unwrap());
        resolver.add_package("b", Version::parse("1.0.0").unwrap());
        resolver.add_package("c", Version::parse("1.0.0").unwrap());
        
        // 添加依赖
        resolver.add_dependency(
            "a",
            Version::parse("1.0.0").unwrap(),
            "b",
            VersionConstraint::parse("^1.0.0").unwrap()
        );
        
        resolver.add_dependency(
            "b",
            Version::parse("1.0.0").unwrap(),
            "c",
            VersionConstraint::parse("^1.0.0").unwrap()
        );
        
        // 解析
        let mut root_deps = HashMap::new();
        root_deps.insert("a".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
        
        let result = resolver.resolve(&root_deps).unwrap();
        
        assert_eq!(result.resolved.len(), 3);
        assert!(result.resolved.contains_key("a"));
        assert!(result.resolved.contains_key("b"));
        assert!(result.resolved.contains_key("c"));
    }
}