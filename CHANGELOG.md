# Changelog

All notable changes to this project will be documented in this file.

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
