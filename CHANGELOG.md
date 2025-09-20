# Changelog

All notable changes to this project will be documented in this file.

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
