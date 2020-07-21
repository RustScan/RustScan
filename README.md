<p align="center">
<img src="pictures/rustscan.png"><br>
Find all open ports <b>fast</b> with Rustscan, automatically pipe them into Nmap. Built with Rust. 
</p>
<p align="center">
<img alt="Crates.io" src="https://img.shields.io/crates/d/rust_scan">
<img alt="AUR version" src="https://img.shields.io/aur/version/rustscan-bin">
</p>
<hr>


| <p align="center"><a href="https://crates.io/crates/rust_scan">🔧 Cargo (Universal) </a></p> | <p align="center"><a href="https://crates.io/crates/rust_scan"> Arch </a></p> | <p align="center"><a href="https://crates.io/crates/rust_scan"> HomeBrew </a></p> | <p align="center"><a href="https://crates.io/crates/rust_scan"> Kali / Debian </p> |
| ---- | ---- | ---- | --- |
| <p align="center"><img src="pictures/rust.png" /></p>  | <p align="center"><img src="pictures/arch.png" /></p> | <p align="center"><img src="pictures/apple.png" /></p> | <p align="center"><img src="pictures/kali.png" /></p> |
| `cargo install rust_scan` | `yay rustscan` | `brew tap brandonskerritt/rustscan && brew install rustscan` | [Read the install guide](https://github.com/brandonskerritt/RustScan/blob/master/README.md#%EF%B8%8F-debian--kali) |

**Note**: You must have Nmap installed.
<hr>

# 🤔 What is this?
1. Find ports quickly using Rustscan (**26 seconds**).
2. Automatically runs `nmap -A -p $ports -vvv` on those ports.
3. Profit???

<table>
  <tr>
  <th>Name</th>
    <th>⚡ Nmap <b>with RustScan</b> ⚡ </th>
    <th>🐢 Nmap 🐢</th>
  </tr>
  <tr>
  <th>Gif</th>
    <td><img src="pictures/with_rustscan.gif" alt="The guy she tells you not to worry about"></td>
    <td><img src="pictures/without_rustscan.gif" alt="You"></td>
  </tr>
  <tr>
  <th>Time</th>
    <td><b>39 seconds</b></td>
    <td><b>17 minutes and 41 seconds</b></td>
  </tr>
    <tr>
  <th>Setup</th>
    <td><ul><li>Set threads to 1000</li></ul></td>
    <td><ul><li>Run nmap with -A for all scripts</li><li>Run Nmap with -p- for all ports</li></ul></td>
  </tr>
</table>

<sub><b>Note</b> The nmap command used was the same for both of them. Nmap -A.</sub><br>

**RustScan takes the Nmap scan down to 39 seconds from 17 minutes and 41 seconds.**

RustScans **only** job is to reduce the friction between finding open ports and inputting them into nmap.

# ✨ Features
* Scans all 64k ports in 26 seconds (on 1k threads).
* Saves you time by automatically piping it into Nmap. No more manual copying and pasting!
* Does one thing and does it well. **Only** purpose is to improve Nmap, not replace it!

# 🔭 Why RustScan?
Why spend time running fast scans and manually copying the ports, or waiting for a 20 minute scan to finish when you can just do all 64k ports in less than a minute?

![gif](/pictures/intro.gif)

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

## 🖥️ Debian / Kali

Download the .deb file from the releases page:
[https://github.com/brandonskerritt/RustScan/releases/tag/1.0.1](https://github.com/brandonskerritt/RustScan/releases/tag/1.0.1)
Run the commpand `dpkg -i` on the file. Note: sometimes you can double click the file to achieve the same result.

## 🍺 HomeBrew

Tap the brew:

```
brew tap brandonskerritt/rustscan
```

Install it:

```
brew install rustscan
```

# 🤸 Usage

```
rustscan -h
```

The format is `rustcan -t 500 -T 1500 192.168.0.1` to scan 192.168.0.1 with 500 threads with a timeout of 1500ms. The timeout is how long RustScan waits for a response until it assumes the port is closed.

# 🎪 Contributing
Please read the [contributing.md file](contributing.md)

# 💻 Other Hacking Projects By This Author
## 🧮 Ciphey
Ciphey is an automated decryption tool using artifical intelligence & natural language processing.
[Check it out here!](https://github.com/ciphey/ciphey)
## Contributors ✨
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://skerritt.blog"><img src="https://avatars3.githubusercontent.com/u/10378052?v=4" width="100px;" alt=""/><br /><sub><b>Brandon</b></sub></a><br /><a href="#infra-brandonskerritt" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="https://github.com/brandonskerritt/RustScan/commits?author=brandonskerritt" title="Tests">⚠️</a> <a href="https://github.com/brandonskerritt/RustScan/commits?author=brandonskerritt" title="Code">💻</a> <a href="#design-brandonskerritt" title="Design">🎨</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
