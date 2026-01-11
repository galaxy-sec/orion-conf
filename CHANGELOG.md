# Changelog

All notable changes to this project will be documented in this file.

## 0.4.2 - 2026-01-11

### Added
- Added `env_load_file` helper function to unify file loading with environment variable evaluation and warning
- Automatic detection and warning for undefined environment variables in all `env_load_*` methods
- Warning messages output to both stderr (via `eprintln!`) and logging system (via `log::warn!`)
- Warning format: `vars not value : VAR1,VAR2,...`
- Three demo examples showcasing the warning functionality:
  - `env_var_warning_simple.rs` - Simple demonstration (recommended)
  - `env_var_warning_demo.rs` - Complete demonstration with 4 scenarios
  - `env_var_with_urls.rs` - Demonstration with complex URL values
- Documentation files:
  - `TESTING_ENV_WARNINGS.md` - Testing guide for environment variable warnings
  - `CHANGELOG_ENV_WARNINGS.md` - Detailed changelog for env warning features

### Changed
- Refactored all `env_load_*` methods to use the unified `env_load_file` function:
  - `env_load_ini` (src/persist.rs:135-139)
  - `env_load_json` (src/persist.rs:176-180)
  - `env_load_toml` (src/persist.rs:216-218)
  - `env_load_yaml` (src/persist.rs:253-257)
- Updated dependency: `orion-variate` from `0.10` to `>=0.10.8, <0.11`
- All display messages and prompts changed to English

### Fixed
- Undefined environment variables now trigger warnings while allowing loading to continue (non-breaking)
- Variables that are not defined remain in `${VAR_NAME}` format instead of causing errors

### Dependencies
- **orion-variate**: Updated to `>=0.10.8, <0.11`
  - v0.10.8 fixes bugs when handling values containing `://` (such as database URLs)
  - Critical update for users working with database connection strings and API endpoints

### Testing
- All 22 unit tests passing
- Added 3 comprehensive demo programs for manual testing
- Added integration tests for URL values with `://` protocol schemes

### Notes
- **Backward Compatible**: v0.4.2 is fully backward compatible with v0.4.1
- **No Migration Needed**: The new warning functionality is additive and does not change existing behavior
- **Recommended Update**: Users working with database URLs should update to benefit from orion-variate v0.10.8 fixes

## 0.4.0

- Added
  - Environment variable substitution support with `env_load_*` methods for all formats
    - `EnvLoadable::env_load_conf()` - Auto-detect format with env var substitution
    - `EnvIniLoad::env_load_ini()` - INI format with env var substitution
    - `EnvJsonLoad::env_load_json()` - JSON format with env var substitution
    - `EnvTomlLoad::env_load_toml()` - TOML format with env var substitution
    - `EnvYamlLoad::env_load_yaml()` - YAML format with env var substitution
  - Comprehensive test coverage for all `env_load_*` methods
  - Support for undefined environment variables (preserved as-is in output)
  - Inline parsing APIs `env_parse_*` for INI/JSON/TOML/YAML plus shared `parse_env_string` helper,
    enabling env substitution from in-memory strings and dedicated tests per format
- Changed
  - All deprecated APIs from v0.3.0 have been removed
  - Trait method implementations now use `load_*` naming exclusively
- Removed
  - Deprecated trait aliases: `Configable`, `Persistable`, `IniAble`, `JsonAble`, `Tomlable`, `Yamlable`, `ValueConfable`
  - Deprecated storage traits: `FileStorage`, `ConfigStorage`, `IniStorage`, `JsonStorage`, `TomlStorage`, `ValueStorage`, `YamlStorage`
  - Deprecated methods: `from_conf()`, `from_ini()`, `from_json()`, `from_toml()`, `from_yaml()`, `from_yml()`, `save_yml()`, `from_valconf()`
  - Deprecated type aliases: `StorageReason`, `SerdeResult`, `SerdeError`, `StorageResult`, `StorageError`
  - Deprecated hook alias: `StorageLoadEvent` (use `LoadHook` instead)
- Fixed
  - Corrected method name in `EnvTomlLoad` implementation (was `env_load_json`, now `env_load_toml`)
- Notes
  - Breaking change: All deprecated v0.3.0 APIs removed. Please update to `load_*` methods before upgrading.
  - Environment variable substitution uses `${VAR_NAME}` syntax, compatible with orion-variate library.
  - Test suite: All 18 tests passing with full format coverage (YAML, TOML, JSON, INI).

## 0.3.0

- Added
  - New trait names: `ConfigIO`, `YamlIO`/`TomlIO`/`JsonIO`/`IniIO`, `TextConfigIO`, `FilePersist`.
  - New load-based APIs: `load_conf`, `load_yaml|toml|json|ini`, `load_valconf`.
  - Error domain: `ConfIOReason`; result aliases: `OrionConfResult<T>`, `OrionConfError`.
  - Docs: `MIGRATION.md` migration guide; `AGENTS.md` contributor guide; README Quick Start & Migration link.
- Changed
  - YAML naming unified: prefer `yaml` (`from_yaml/save_yaml`); examples/docs use `load_*` and new trait names.
  - `ConfigIO` default behavior still follows priority YAML > TOML > JSON > INI.
- Deprecated
  - Old trait names: `Configable`, `Yamlable`, `Tomlable`, `JsonAble`, `IniAble`, `ValueConfable`, `Persistable`.
  - Old methods: `from_conf`, `from_yaml|yml|toml|json|ini`, `from_valconf`.
  - Storage alias traits: `FileStorage`, `ConfigStorage`, `IniStorage`, `JsonStorage`, `TomlStorage`, `ValueStorage`, `YamlStorage`.
  - Load hook alias: `StorageLoadEvent` (use `LoadHook`).
  - Type aliases: `SerdeResult`, `SerdeError`, `StorageResult`, `StorageError` (use `OrionConfResult`, `OrionConfError`).
- Removed
  - Error variant `Brief(String)` replaced by `Other(String)`. If you matched on `Brief`, update to `Other`.
- Notes
  - No runtime behavior change; format features remain opt-in; default features are empty.
  - See `MIGRATION.md` for one-line replacements and verification commands.
