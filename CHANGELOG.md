# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.5] - 2025-1-20

### Changed

* 更新 rusqlite 到 ~0.33
* 更新 cppjieba 到 v5.4.0

## [0.3.4] - 2024-10-7

### Added

* 添加 highlight 相关 ffi 入口点

## [0.3.3] - 2024-9-13

### Fixed

* 修复 MSVC 编译 D8021 错误

## [0.3.2] - 2024-9-5

### Fixed

* 优化 `release_dict` 行为，不始终覆盖文件

## [0.3.1] - 2024-7-25

### Changed

* 更新依赖
* 修改 `sqlite3_simple_init` 方法签名

## [0.3.0] - 2024-5-20

### Added

* 添加 `disable_auto_extension` 方法
* 添加更多 ffi 接口

## [0.2.2] - 2024-5-20

### Added

* 添加 hello example

## [0.2.1] - 2024-4-16

### Changed

* 优化文档
* 测试 `sqlcipher`

## [0.2.0] - 2024-4-16

### Added

* Bundled 构建，支持离线
* 添加 `jieba` feature，可选编译

### Changed

* 重构 API

## [0.1.0] - 2024-4-15

### Added

* 支持 `simple v0.4.0`
