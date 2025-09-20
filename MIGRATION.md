# Migration Guide (0.2.x → 0.3.0)

This release keeps source compatibility but introduces deprecation warnings to steer toward clearer names and IO semantics. No runtime behavior changes; default format priority remains YAML > TOML > JSON > INI.

## What Changed
- Trait names (new preferred → old deprecated alias):
  - `ConfigIO` ← `Configable`
  - `YamlIO`/`TomlIO`/`JsonIO`/`IniIO` ← `Yamlable`/`Tomlable`/`JsonAble`/`IniAble`
  - `TextConfigIO` ← `ValueConfable`
  - `FilePersist` ← `Persistable`
- IO method names:
  - New load_ methods: `load_conf`, `load_yaml|toml|json|ini`, `load_valconf`
  - Old from_ methods are deprecated: `from_conf`, `from_yaml|yml|toml|json|ini`, `from_valconf`
  - YAML: `from_yml` is deprecated; use `load_yaml`
- Storage alias traits are deprecated: `FileStorage`, `ConfigStorage`, `IniStorage`, `JsonStorage`, `TomlStorage`, `ValueStorage`, `YamlStorage`
- Load hook: use `LoadHook` (old `StorageLoadEvent` is deprecated alias)
 - Error kind variant: `Brief(String)` → `Other(String)` (old variant kept but deprecated)

## How To Migrate
- Imports
  - `use orion_conf::Configable;` → `use orion_conf::ConfigIO;`
  - `use orion_conf::{JsonAble, Yamlable, Tomlable, IniAble};` → `use orion_conf::{JsonIO, YamlIO, TomlIO, IniIO};`
- Loading
  - `T::from_conf(p)` → `T::load_conf(p)`
  - `T::from_yaml(p)` or `T::from_yml(p)` → `T::load_yaml(p)`
  - `T::from_toml(p)` → `T::load_toml(p)`
  - `T::from_json(p)` → `T::load_json(p)`
  - `T::from_ini(p)` → `T::load_ini(p)`
  - `T::from_valconf(p)` → `T::load_valconf(p)`
- Saving
  - No change to `save_conf`, `save_yaml|toml|json|ini`, `save_valconf`
- Trait bounds/impls
  - Replace `*_Storage` bounds with the direct IO traits (e.g., `T: JsonIO<_>`)
  - Replace `StorageLoadEvent` with `LoadHook`

## Verify
- Feature-gated build/examples
  - `cargo build --features formats`
  - `cargo build --examples --features yaml,json,toml,ini`
- Lints & tests
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-features -- --test-threads=1`

If you need a long-lived transition, the deprecated APIs remain in 0.3.x but may be removed in a future 0.5/1.0 release.
