# new‑web‑cores‑archive 🌐📦

[![GitHub stars](https://img.shields.io/github/stars/Shiro-nn/new-web-cores-archive?style=social)](https://github.com/Shiro-nn/new-web-cores-archive/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Shiro-nn/new-web-cores-archive?style=social)](https://github.com/Shiro-nn/new-web-cores-archive/network/members)
[![GitHub issues](https://img.shields.io/github/issues/Shiro-nn/new-web-cores-archive)](https://github.com/Shiro-nn/new-web-cores-archive/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/Shiro-nn/new-web-cores-archive)](https://github.com/Shiro-nn/new-web-cores-archive/commits)
[![License: MIT](https://img.shields.io/github/license/Shiro-nn/new-web-cores-archive)](LICENSE)
[![Status: Archived](https://img.shields.io/badge/status-archived-lightgrey.svg)](https://github.com/Shiro-nn/new-web-cores-archive)

![Repo Stats](https://github-readme-stats.vercel.app/api/pin/?username=Shiro-nn\&repo=new-web-cores-archive)

> **new‑web‑cores‑archive** — набор микросервисов, который я разрабатывал в 2024‑2025 гг. для одного игрового проекта: аутентификация, CDN‑прокси и GeoIP‑API. В марте 2025 года разработка остановлена, репозиторий переведён в **архивный режим**. Код остаётся в открытом доступе «как есть» — без гарантий поддержки и новых релизов.

---

## 📂 Состав репозитория

| Директория  | Язык/стек                                          | Краткое описание                                                                                                              |
| ----------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| **`auth`**  | **Rust** (`actix‑web`, `mongodb`, `include‑crypt`) | REST‑API для регистрации/логина, выдаёт JWT, шифрует конфиги прямо в бинарнике.                                               |
| **`cdn`**   | **Rust** (`actix‑web`, `tokio`)                    | Лёгкий обратный прокси и кэш для статики (аналог mini‑CDN): проверка ETag, сжатие brotli/gzip, пассивное кеширование на диск. |
| **`geoip`** | **Node.js** (`express`, `maxmind`)                 | HTTP‑эндпоинт `/geoip?ip=` возвращает ISO‑коды страны/города и координаты по базе MaxMind GeoLite2.                           |

> **Важно:** все три сервиса запускаются **независимо** и не связаны единой оркестрацией — каждый имеет собственный `Cargo.toml` или `package.json`.

---

## 🚀 Быстрый старт (локально)

### auth

```bash
git clone https://github.com/Shiro-nn/new-web-cores-archive.git
cd new-web-cores-archive/auth
cargo run --release            # по умолчанию 0.0.0.0:8080
```

Переменные окружения (`.env`):

```
MONGO_URI=<зашифрованный URI>
JWT_SECRET=<любая_строка>
PORT=8080
```

### cdn

```bash
cd ../cdn
cargo run --release            # стартует на 0.0.0.0:8090
```

Параметры настраиваются через `config.json` (примеры в директории).

### geoip

```bash
cd ../geoip
npm install
node index.js                  # порт 3000
```

Для обновления MaxMind GeoLite2 положите файл `GeoLite2-City.mmdb` в корень `geoip/data`.

---

## 🧩 Мини‑архитектура

```mermaid
graph LR
    subgraph Public
        A[Frontend / Клиенты]
    end
    A -->|HTTPS| B(auth)
    A -->|HTTPS| C(cdn)
    A -->|HTTPS| D(geoip)
    B -->|MongoDB| E[(Database)]
    C -->|MongoDB| E[(Database)]
```

* **auth** отвечает за аутентификацию и генерирует JWT, который клиенты передают далее.
* **cdn** прозрачно проксирует статические файлы, проверяя подписи JWT (опционально).
* **geoip** используется внутрисетевыми сервисами для гео‑таргетинга.

---

## 🛠️ Системные требования

* **Rust 1.75+** и `cargo` для `auth` и `cdn`.
* **Node.js 18+** для `geoip`.
* **MongoDB 6+** (если хотите полноценную регистрацию/логирование).

---

## 🤝 Вклад

Репозиторий **архивирован** (март 2025), поэтому PR принимаются только на критические баг‑фиксы или обновления зависимостей. Для развития идей — форкайте и экспериментируйте.

---

## ⚖️ Лицензия

Код распространяется под лицензией **MIT**. Используйте свободно, но без каких‑либо гарантий.

> Спасибо за интерес! Надеюсь, примеры кода окажутся полезными в ваших проектах.
