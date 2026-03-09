# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2026-3-10

### Changed

* 更新 `simple` 到 [v0.7.1](https://github.com/wangfenjin/simple/commit/4ed008934495fc55ff4bf6620bba58311988b23e)
* 更新 `cppjieba` 到 [v5.6.2](https://github.com/yanyiwu/cppjieba/commit/1a968ac58937ad6dc19813a1adb99516c1646d0b)
* 在编译时生成 cmrc

## [0.7.1] - 2026-2-22

### Fixed

*  修复关闭 `jieba` 功能时的编译错误

## [0.7.0] - 2026-2-21

### Added

* 添加 `set_pinyin_dict` 允许自定义 `pinyin.txt`

### Changed

* 更新 `simple` 到 [v0.7.0](https://github.com/wangfenjin/simple/commit/683a0afa7a20dddd4857688a4468834c21a15798)
* 重命名 `release_dict` 为 `release_jieba_dict`
* 重命名 `set_dict` 为 `set_jieba_dict`

## [0.6.1] - 2025-8-10

### Changed

* 更新 `simple` 补丁 [b88f87035795cfdda71ba8629967973ed00e404c](https://github.com/wangfenjin/simple/tree/b88f87035795cfdda71ba8629967973ed00e404c)
* 关闭 `better_embedded` 默认功能

## [0.6.0] - 2025-5-27

### Changed

* 更新 `rusqlite` 到 `>=0.32,<1.0`
* 更新 `cppjieba` 到 [v5.6.0](https://github.com/yanyiwu/cppjieba/commit/294755fab1ea0c2544442b27cb7d2ad9bcc4ba01)

## [0.5.0] - 2025-4-24

### Changed

* 更新 `rusqlite` 到 ~0.35
* 更新 `simple` 到 [v0.5.2](https://github.com/wangfenjin/simple/releases/tag/v0.5.2)
* 更新 `cppjieba` 到 [v5.5.0](https://github.com/yanyiwu/cppjieba/commit/3732abc0e5548c96b4a6ea55113a02df97acf761)

## [0.4.0] - 2025-3-6

### Changed

* 使用 edition 2024
* MSRV 为 1.85.0

## [0.3.7] - 2025-3-6

### Changed

* 更新 `rusqlite` 到 ~0.34

## [0.3.6] - 2025-1-25

### Changed

* 更新 `cppjieba` 到 [v5.4.0](https://github.com/yanyiwu/cppjieba/commit/9b45e084a3153b9a60ead6c8fecc97345c17da15)

## [0.3.5] - 2025-1-20

### Changed

* 更新 `rusqlite` 到 ~0.33

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

* 更新 `rusqlite` 到 ~0.32
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

* 支持 `simple` [v0.4.0](https://github.com/wangfenjin/simple/releases/tag/v0.4.0)
