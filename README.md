# 🔥 Abdal HTTP HEAD Flood


<div align="center">
  <img src="scr.jpg" alt="Abdal HTTP HEAD Flood" >
</div>


This software is part of the Abdal arsenal, which belongs to the Abdal Security Group, led by Ebrahim Shafiei (EbraSha).

A high-performance, customizable HTTP HEAD flood testing tool written in **Rust**.  
Designed for cybersecurity research, network stress testing, and performance benchmarking.

## 🎙️ Translation

[English](README.md) | [فارسی](README.fa.md)

---

## ⚙️ Features

- ⚡ **High-performance async engine** powered by [`tokio`] and [`reqwest`]
- 🧠 **Fully customizable headers**:
  - `User-Agent` Spoofing browser and real attacker identity
  - `Referer` Spoofing referrer
  - `Host` Spoofing host section
  - `Connection` For creating additional pressure
  - `X-Forwarded-For`, `Client-IP` For IP spoofing
- 🎯 **Target specification**:
  - Domain name or IP address
  - HTTP/HTTPS support
- 📥 **Interactive command-line interface**
  - User-friendly prompts
  - Minimal typing (menu-based selection)
- 🎨 **Cyberpunk-style ASCII banner** with terminal colors
- 🔄 **Multithreaded parallel execution**
- 🔍 **Verbose logging** (optional per-request logging)
- 🧪 Designed for **red teaming**, **stress testing**, and **defensive analysis**
- ✅ This tool is fully cross-platform and works seamlessly on **Windows**, **Linux**, and **macOS**.
---

## 🧩 Example Usage

Interactive mode will guide you through:
- Entering target (IP/URL)
- Number of total requests
- Thread count (concurrency)
- Enabling/disabling per-request logs
- IP spoofing options
- Header customization (User-Agent, Host, Referer, Connection)

### Example Usage 💥 High Load Stress Test Mode
🔧 Configuration Profile – Example Target: https://ebrasha.com

✅ Recommended Configuration:
```shell
Enter target URL or IP: https://ebrasha.com
Enter total number of requests: 50000000
Enter concurrency level (threads): 300
Show each request? (y/n): y
Show server responses? (y/n): y
Provide custom User-Agent list? (y/n): n
Provide custom Referer list? (y/n): n
Spoof IP headers? (y/n): n

Select Connection header:
1. keep-alive
2. close
3. upgrade
4. None (don’t include Connection header)
   Enter option [1-4]: 2
   Use specific Host header? (y/n): https://ebrasha.com

```

## ⚠️ Important Notes about Concurrency Level

### 🚫 Do NOT raise concurrency level without proper reasoning!

- Setting `concurrency` too high (e.g., above 500) may:
  - **Freeze or heavily slow down your machine**
  - **Exhaust CPU and memory resources**
  - Cause excessive **timeouts, errors, and inaccurate test results**

> ✅ Always choose `concurrency` based on your hardware capability.


## 🖥️ Suggested Concurrency Based on Your System

| 💻 System Specs                  | 🔢 Recommended Concurrency |
|----------------------------------|-----------------------------|
| 2-core CPU + 4GB RAM             | 50–100                      |
| 4-core CPU + 8GB RAM             | 100–300                     |
| Multi-core CPU + 16GB RAM        | 300–500 or higher           |


---

## 🧠 Architecture

- Written in pure **Rust**
- Uses `tokio` for async concurrency
- Multi-threaded request dispatcher
- Random header generation logic using `rand`
- Modular design: easily extendable

---

## 📦 Dependencies

- [`reqwest`](https://crates.io/crates/reqwest)
- [`tokio`](https://crates.io/crates/tokio)
- [`rand`](https://crates.io/crates/rand)
- [`colored`](https://crates.io/crates/colored) – for terminal art
- `std::io`, `std::sync::Arc`, `std::time::Duration`

## ⚠️ Ethical Usage Warning
This tool is meant for **legal and ethical security testing only**. Always:
- Obtain proper authorization before testing any website
- Follow responsible disclosure practices
- Respect privacy and data protection laws
- Use this tool only on systems you are authorized to test

## 🐛 Reporting Issues
If you encounter any issues or have configuration problems, please reach out via email at Prof.Shafiei@Gmail.com. You can also report issues on GitLab or GitHub.

## ❤️ Donation
If you find this project helpful and would like to support further development, please consider making a donation:
- [Donate Here](https://alphajet.ir/abdal-donation)

## 🤵 Programmer
Handcrafted with Passion by **Ebrahim Shafiei (EbraSha)**
- **E-Mail**: Prof.Shafiei@Gmail.com
- **Telegram**: [@ProfShafiei](https://t.me/ProfShafiei)

## 📜 License
This project is licensed under the GPLv2 or later License. 