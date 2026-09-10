# Запуск

- Установить Rust и Dioxus: https://dioxuslabs.com/learn/0.7/getting_started/. В качестве целевой плафтормы рекомендуется:

```bash
rustup target add x86_64-pc-windows-msvc
```

- Запустить PostgreSQL, исправить `config.toml`
- Запустить Dioxus:

```bash
dx serve --web
```

```bash
dx serve --desktop
```
