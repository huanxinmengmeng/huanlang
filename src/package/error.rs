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

//! 包管理器错误类型

use std::fmt;

/// 包管理器错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum PackageError {
    /// 配置文件错误
    ConfigError {
        message: String,
        path: Option<String>,
    },
    /// 依赖解析错误
    DependencyError {
        message: String,
        conflicts: Option<Vec<(String, String)>>,
    },
    /// 注册表错误
    RegistryError {
        message: String,
        code: Option<u16>,
    },
    /// 网络错误
    NetworkError {
        message: String,
        url: Option<String>,
    },
    /// 构建错误
    BuildError {
        message: String,
        command: Option<String>,
    },
    /// 包错误
    PackageError {
        message: String,
        package: Option<String>,
    },
    /// IO 错误
    IoError {
        message: String,
        path: Option<String>,
    },
    /// 版本错误
    VersionError {
        message: String,
        version: Option<String>,
    },
    /// 认证错误
    AuthError {
        message: String,
    },
    /// 未知错误
    UnknownError {
        message: String,
    },
}

impl PackageError {
    pub fn config_error(message: &str, path: Option<&str>) -> Self {
        PackageError::ConfigError {
            message: message.to_string(),
            path: path.map(|p| p.to_string()),
        }
    }

    pub fn dependency_error(message: &str, conflicts: Option<Vec<(String, String)>>) -> Self {
        PackageError::DependencyError {
            message: message.to_string(),
            conflicts,
        }
    }

    pub fn registry_error(message: &str, code: Option<u16>) -> Self {
        PackageError::RegistryError {
            message: message.to_string(),
            code,
        }
    }

    pub fn network_error(message: &str, url: Option<&str>) -> Self {
        PackageError::NetworkError {
            message: message.to_string(),
            url: url.map(|u| u.to_string()),
        }
    }

    pub fn build_error(message: &str, command: Option<&str>) -> Self {
        PackageError::BuildError {
            message: message.to_string(),
            command: command.map(|c| c.to_string()),
        }
    }

    pub fn package_error(message: &str, package: Option<&str>) -> Self {
        PackageError::PackageError {
            message: message.to_string(),
            package: package.map(|p| p.to_string()),
        }
    }

    pub fn io_error(message: &str, path: Option<&str>) -> Self {
        PackageError::IoError {
            message: message.to_string(),
            path: path.map(|p| p.to_string()),
        }
    }

    pub fn version_error(message: &str, version: Option<&str>) -> Self {
        PackageError::VersionError {
            message: message.to_string(),
            version: version.map(|v| v.to_string()),
        }
    }

    pub fn auth_error(message: &str) -> Self {
        PackageError::AuthError {
            message: message.to_string(),
        }
    }

    pub fn unknown_error(message: &str) -> Self {
        PackageError::UnknownError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageError::ConfigError { message, path } => {
                write!(f, "配置错误: {}", message)?;
                if let Some(p) = path {
                    write!(f, " (文件: {})", p)?;
                }
                Ok(())
            }
            PackageError::DependencyError { message, conflicts } => {
                write!(f, "依赖解析错误: {}", message)?;
                if let Some(c) = conflicts {
                    write!(f, " (冲突: {:?})", c)?;
                }
                Ok(())
            }
            PackageError::RegistryError { message, code } => {
                write!(f, "注册表错误: {}", message)?;
                if let Some(c) = code {
                    write!(f, " (代码: {})", c)?;
                }
                Ok(())
            }
            PackageError::NetworkError { message, url } => {
                write!(f, "网络错误: {}", message)?;
                if let Some(u) = url {
                    write!(f, " (URL: {})", u)?;
                }
                Ok(())
            }
            PackageError::BuildError { message, command } => {
                write!(f, "构建错误: {}", message)?;
                if let Some(c) = command {
                    write!(f, " (命令: {})", c)?;
                }
                Ok(())
            }
            PackageError::PackageError { message, package } => {
                write!(f, "包错误: {}", message)?;
                if let Some(p) = package {
                    write!(f, " (包: {})", p)?;
                }
                Ok(())
            }
            PackageError::IoError { message, path } => {
                write!(f, "IO错误: {}", message)?;
                if let Some(p) = path {
                    write!(f, " (路径: {})", p)?;
                }
                Ok(())
            }
            PackageError::VersionError { message, version } => {
                write!(f, "版本错误: {}", message)?;
                if let Some(v) = version {
                    write!(f, " (版本: {})", v)?;
                }
                Ok(())
            }
            PackageError::AuthError { message } => {
                write!(f, "认证错误: {}", message)
            }
            PackageError::UnknownError { message } => {
                write!(f, "未知错误: {}", message)
            }
        }
    }
}

impl std::error::Error for PackageError {}

impl From<std::io::Error> for PackageError {
    fn from(error: std::io::Error) -> Self {
        PackageError::IoError {
            message: error.to_string(),
            path: None,
        }
    }
}

impl From<std::time::SystemTimeError> for PackageError {
    fn from(error: std::time::SystemTimeError) -> Self {
        PackageError::IoError {
            message: error.to_string(),
            path: None,
        }
    }
}

pub type PackageResult<T> = Result<T, PackageError>;
