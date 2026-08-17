<div align="center">

# ⚡ RustPulse

**A fast, lightweight, concurrent load testing CLI tool built in Rust.**

Send concurrent requests to any website, measure real response performance under load, and get a clean, instant report — right from your terminal.

[![Rust](https://img.shields.io/badge/Rust-1.97.1-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Cargo-1.97.1-orange?logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Tokio](https://img.shields.io/badge/Async-Tokio-blue?logo=rust&logoColor=white)](https://tokio.rs/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](#-license)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#-contributing)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)](#)

</div>

---

## 📌 Overview

**RustPulse** is a minimal, no-frills CLI tool that helps developers and DevOps engineers check how a website or API **really performs under concurrent load** — without needing a heavyweight testing suite.

It:

- ✅ Checks whether a target host is live before running any test.
- ⚡ Spawns concurrent async requests (`tokio::spawn`) to a target URL.
- 📊 Measures real response times under load — not simulated numbers.
- 🧮 Calculates **Mean**, **Standard Deviation**, **Min/Max**, and a **Performance Score**.
- 🖥️ Prints a clean, human-readable report in the terminal.
- 💾 Saves all results to a CSV file, named after the target host.

---

## 🎥 Demo

```bash
$ cargo run -- -url https://example.com -count 50

Host is chekd : Done
Wait, checking real response performance under load..

╔══════════════════════════════════════╗
║          LOAD TEST REPORT             ║
╚══════════════════════════════════════╝
 Target URL       : https://example.com
 Total Requests   : 50
 Success          : 50
 Failed           : 0
 Success Rate     : 100.00%
----------------------------------------
 Min Response     : 0.812s
 Max Response     : 1.503s
 Mean Response    : 1.143s
 Std Deviation    : 0.187s
----------------------------------------
 Performance Score: 87.49 / 100 🟡 Good
========================================

File Save Done
```

Generated CSV output (`example.csv`):

```csv
id,response_time
1,1.021
2,0.998
3,1.204
...
```

---

## 🛠️ Tech Stack

| Technology | Purpose |
|---|---|
| **Rust** | Core language — performance and safety |
| **Tokio** | Async runtime — concurrent task spawning (`tokio::spawn`, `#[tokio::main]`) |
| **Regex** | URL parsing — extracting the host name |
| **std::sync (Arc, Mutex)** | Thread-safe shared state (error counter) |
| **CSV** | Persisting results to disk |

---

## 📁 Project Structure

```
RustPulse/
├── Cargo.toml            # Project dependencies & metadata
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs            # CLI entry point — argument parsing (-url, -count)
│   ├── core/
│   │   ├── mod.rs
│   │   └── core.rs         # Core logic: concurrent requests, performance analytics, reporting
│   └── libs/
│       ├── mod.rs
│       └── funtion.rs      # Shared helpers: request handling, CSV writer
└── *.csv                  # Generated report output files
```

---

## ⚙️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) `>= 1.97.1`
- Cargo (bundled with Rust)

```bash
rustc -V   # rustc 1.97.1
cargo -V   # cargo 1.97.1
```

### Installation

```bash
git clone https://github.com/<your-username>/RustPulse.git
cd RustPulse
cargo build --release
```

### Usage

```bash
cargo run -- -url <TARGET_URL> -count <REQUEST_COUNT>
```

**Example:**

```bash
cargo run -- -url https://example.com -count 50
```

| Flag | Description | Default |
|---|---|---|
| `-url` | Target URL to test | — (required) |
| `-count` | Number of concurrent requests to send | `10` |

---

## 📊 Report Metrics

| Metric | Formula |
|---|---|
| Mean (μ) | Σx / n |
| Standard Deviation (σ) | √(Σ(x − μ)² / n) |
| Performance Score | (target / mean) × 100, capped at 100 |

---

## 🗺️ Roadmap

- [ ] Configurable performance target via CLI flag
- [ ] JSON output format
- [ ] Request timeout & retry options
- [ ] Percentile-based metrics (P95, P99)

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to check the [issues page](../../issues) or open a pull request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 👤 Author

**Owen Lasith Ruwantha Amarawansha**

---

## 📄 License

This project is licensed under the **MIT License** — free to use, modify, and distribute for personal and educational purposes.

---

<div align="center">

If you find this project useful, consider giving it a ⭐ on GitHub!

</div>
