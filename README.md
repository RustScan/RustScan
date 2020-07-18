<p align="center">
<img src="pictures/rustscan.png"><br>
Find all open ports <b>fast</b> with Rustscan, automatically pipe them into Nmap. Built with Rust. 
<br>
<img alt="Crates.io" src="https://img.shields.io/crates/d/rust_scan">
</p>
<hr>


| <p align="center"><a href="https://crates.io/crates/rust_scan">🔧 Cargo (Universal) </a></p> | <p align="center"><a href="https://pypi.org/project/ciphey"> Arch </a></p> | <p align="center"><a href="https://pypi.org/project/ciphey"> HomeBrew </a></p> | <p align="center"><a href="https://pypi.org/project/ciphey"> Kali / Debian </p> |
| ---- | ---- | ---- | --- |
| <p align="center"><img src="pictures/rust.png" /></p>  | <p align="center"><img src="pictures/arch.png" /></p> | <p align="center"><img src="pictures/apple.png" /></p> | <p align="center"><img src="pictures/kali.png" /></p> |
| `cargo install rust_scan` | `yay rustscan` | `brew install rustscan` | `dpkg -i rustscan.deb` |

**Note**: You must have Nmap installed.
<hr>

# TODO
* debian package
* terminal gifs

# 🤔 What is this?
If you are a competitive CTF player and often find yourself running masscan / a basic nmap scan before running a more comprehensive scan, this tool is for you.
1. Find ports quickly using Rustscan (**27 seconds on average**).
2. Automatically runs `nmap -A -sV -p $ports -vvv` on those ports.
3. Profit???

[ GIFS HERE ]

Rustscans **only** job is to reduce the friction between finding open ports and inputting them into nmap.

# ✨ Features
* Scans all 64k ports in 27 seconds (on 1k threads).
* Saves you time by automatically piping it into Nmap. No more manual copying and pasting!
* Does one thing and does it well. **Only** purpose is to improve Nmap, not replace it!

# 🔭 Why RustScan?

## 🔬 Without RustScan
* Gif here comparison

## 📊 RustScan vs Nmap vs MassScan

| **Name**                                                                                   | RustScan | Nmap | Masscan |
| ------------------------------------------------------------------------------------------ | -------- | ---- | ------- |
| Fast                                                                                       | ✅        | ❌    | ✅       |
| Actually useful                                                                            | ❌        | ✅    | ❌       |
| Realises it's not useful, and pipes the only useful data into the only useful port scanner | ✅        | ❌    | ❌       |


## 🙋 FAQ
> I think this would be a great port scanner on its own without Nmap!

No. If you want a fast port scanner, use Masscan.
> I have this great idea for a script to get information on ports / hosts

Great. Contribute it to Nmap! :D
> Not everyone has nmap installed....

If you're a pentester, then yes, you have Nmap installed. 

> I want to contribute!

Great! I'd love some help with this. Read the [contributing.md file](contributing.md) file for more information!

# 📖 Full Installation Guide
**You need Nmap**. If you have Kali Linux or Parrot OS installed, you already have Nmap. If not, [follow the nmap install guide](https://nmap.org/download.html).

The easiest way to install RustScan is to use one of the packages provided for your system, such as HomeBrew or Yay for Arch Linux.

The most universal way is to use `cargo`, Rust's built in package manager (think Pip but for Rust). [Follow this guide to installing Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

If you face any issues at all, please leave a GitHub issue. I have only tested this on Linux, so there may be issues for Mac OS or Windows. 

# 🎪 Contributing
Please read the [contributing.md file](contributing.md)

# 💻 Other Hacking Projects By This Author
## 🧮 Ciphey
Ciphey is an automated decryption tool using artifical intelligence & natural language processing.
[Check it out here!](https://github.com/ciphey/ciphey)

