<p align="center">
➡️
<a href="http://discord.skerritt.blog">Discord</a> |
 <a href="https://github.com/RustScan/RustScan/wiki/Installation-Guide">Installation Guide</a> |
 <a href="https://github.com/RustScan/RustScan#-usage">Usage Guide</a>
 ⬅️
<br>
<img src="pictures/rustscan.png" height=400px width=400px>
</p>
<p align="center">
<u><b> The Modern Port Scanner. </b></u><br><b>Fast, smart, effective.</b> 
</p>
<p align="center">
<img alt="AUR version" src="https://img.shields.io/aur/version/rustscan">
<img src="https://img.shields.io/badge/Built%20with-Rust-Purple">
<img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/rustscan/rustscan/total?label=GitHub%20Downloads">
<img alt="Crates.io" src="https://img.shields.io/crates/d/rustscan?label=Cargo%20Downloads">
<img alt="Discord" src="https://img.shields.io/discord/754001738184392704">
<img alt="Actions" src="https://github.com/RustScan/RustScan/workflows/Continuous%20integration/badge.svg?branch=master">
</p>
<hr>

| <a href="https://hub.docker.com/r/cmnatic/rustscan">🐋 Docker (Recommended)</a> | <a href="https://github.com/RustScan/RustScan/releases">👩‍💻 Kali / Debian | <a href="https://aur.archlinux.org/packages/rustscan/">🏗️ Arch </a> | <a href="https://crates.io/crates/rustscan">🔧 Homebrew </a> |
|:-----------------------------------------------------------------------------------------------------------------:|:---------------------------------------------------------------------------------------------------------------:|:---------------------------------------------------------------------------------------------------------------:|:---------------------------------------------------------------------------------------------------------------:|
| <p align="center"><img src="https://github.com/RustScan/RustScan/blob/master/pictures/docker.png?raw=true" /></p> | <p align="center"><img src="https://github.com/RustScan/RustScan/blob/master/pictures/kali.png?raw=true" /></p> | <p align="center"><img src="https://github.com/RustScan/RustScan/blob/master/pictures/arch.png?raw=true" /></p> | <p align="center"><img src="https://raw.githubusercontent.com/RustScan/RustScan/master/pictures/apple.png" /></p> |
| `docker pull rustscan/rustscan:2.1.1` [Usage](https://github.com/RustScan/RustScan/wiki/Installation-Guide#docker-whale) | [Read the install guide](https://github.com/RustScan/RustScan/wiki/Installation-Guide#%EF%B8%8F-debian--kali)     | `yay -S rustscan` | `brew install rustscan` |

<hr>

# 🤔 What is this?

![fast](pictures/fast.gif)

The Modern Port Scanner. **Find ports quickly (3 seconds at its fastest)**. Run scripts through our scripting engine (Python, Lua, Shell supported).

# ✨ Features

- Scans all 65k ports in **3 seconds**.
- Full scripting engine support. Automatically pipe results into Nmap, or use our scripts (or write your own) to do whatever you want.
- Adaptive learning. RustScan improves the more you use it. No bloated machine learning here, just basic maths.
- The usuals you would expect. IPv6, CIDR, file input and more.
- Automatically pipes ports into Nmap.

## ‼️ Important Links

| Installation Guide                                                                     | Documentation                                            | Discord                                  |
| -------------------------------------------------------------------------------------- | -------------------------------------------------------- | ---------------------------------------- |
| 📖 [Installation Guide](https://github.com/RustScan/RustScan#-full-installation-guide) | 📚 [Documentation](https://rustscan.github.io/RustScan/) | 🦜 [Discord](http://discord.skerritt.blog) |

## 🙋 Table of Contents

- 📖 [Installation Guide](https://github.com/RustScan/RustScan/wiki/Installation-Guide)
- 🐋 [Docker Usage](https://github.com/RustScan/RustScan/wiki/Installation-Guide)
- 🦜 [Discord](http://discord.skerritt.blog)
- 🤸 [Usage](https://github.com/RustScan/RustScan/wiki/Usage)
- 🎪 [Community](https://github.com/RustScan/RustScan#-community)

# 🔭 Why RustScan?

RustScan is a modern take on the port scanner. Sleek & fast. All while providing extensive extendability to you.

Not to mention RustScan uses Adaptive Learning to improve itself over time, making it the best port scanner for **you**.

## 🧋 Speed

![fast](pictures/fast.gif)

Speed is guaranteed via RustScan. However, if you want to run a slow scan due to stealth that is possible too.

Firstly, let's talk code.

We have tests that check to see if RustScan is significantly slower than the previous version. If it is, the continuous integration fails and we can't commit code to master unless we make it faster.

[HyperFine](https://github.com/sharkdp/hyperfine) is used to monitor RustScan's performance over time to answer the question "Are we getting faster? Are we getting slower?".

Every pull request is reviewed by 1 person, but more often than not 2 people review it. We test it manually and make sure the code doesn't affect performance negatively.

[Read more here](https://github.com/RustScan/RustScan/wiki/Increasing-Speed-&-Accuracy).

## ⚙️ Extensible 

![scripts](pictures/scripts.gif)

_RustScan piping results into the custom Python script_

RustScan has a new scripting engine which allows anyone to write scripts in most languages. Python, Lua, Shell are all supported.

Want to take your found ports and pipe them into Nmap for further analysis? That's possible. Want to run `smb-enum` if SMB is found open? Possible.

The possibilities are endless -- and you can write scripts in whatever language you feel comfortable with.

[Read more here](https://github.com/RustScan/RustScan/wiki/RustScan-Scripting-Engine).

## 🌊 Adaptive

![adaptive](pictures/adaptive.gif)

_RustScan automatically fine-tuning itself to match the host OS_.

RustScan has a cool set of features called "Adaptive Learning". These features "learn" about the environment you are scanning and how _you_ use RustScan to **improve itself over time**.

This is an umbrella term we use for any feature that fits this criteria. The list is constantly changing, so [check out our wiki for more information](https://github.com/RustScan/RustScan/wiki/Adaptive-Learning).

## 👩‍🦯 Accessible

![fast](pictures/accessible.gif)

RustScan is one of the first penetration testing tools that aims to be entirely accessible. 

[Most penetration testing tools are not accessible](https://bees.substack.com/p/making-hacking-accessible), which negatively affects the whole industry.

RustScan has continuous integration testing that aims to make sure it is accessible, and we are constantly working on ways to improve our accessibility and make sure _everyone_ can use RustScan.

# 📖 Full Installation Guide

You can find our guide [here](https://github.com/RustScan/RustScan/wiki/Installation-Guide).

## 🦊 Community Distributions

Here are all of RustScan's community distributions.

If you maintain a community distribution and want it listed here, leave an issue / pull request / Discord message or however you want to let us know.

- [OpenSuse](https://software.opensuse.org/package/rustscan?search_term=rustscan)
- [Fedora/CentOS](https://copr.fedorainfracloud.org/coprs/atim/rustscan/)

[![Packaging status](https://repology.org/badge/vertical-allrepos/rustscan.svg)](https://repology.org/project/rustscan/versions)

# 🤸 Usage

We have 2 usage guides. [Basic Usage](https://github.com/RustScan/RustScan/wiki/Usage) and [Things you may want to do](https://github.com/RustScan/RustScan/wiki/Things-you-may-want-to-do-with-RustScan-but-don't-understand-how).

We also have documentation about our config file [here](https://github.com/RustScan/RustScan/wiki/Config-File).

# 🎪 Community

[Read this to learn how to contribute](https://github.com/RustScan/RustScan/wiki/Contributing).

## Contributors ✨

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-26-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://skerritt.blog"><img src="https://avatars3.githubusercontent.com/u/10378052?v=4" width="100px;" alt=""/><br /><sub><b>Brandon</b></sub></a><br /><a href="#infra-brandonskerritt" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="https://github.com/RustScan/RustScan/commits?author=brandonskerritt" title="Tests">⚠️</a> <a href="https://github.com/RustScan/RustScan/commits?author=brandonskerritt" title="Code">💻</a> <a href="#design-brandonskerritt" title="Design">🎨</a></td>
    <td align="center"><a href="https://sakiir.ovh"><img src="https://avatars1.githubusercontent.com/u/9950578?v=4" width="100px;" alt=""/><br /><sub><b>SakiiR</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=SakiiR" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3ASakiiR" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/smackhack"><img src="https://avatars2.githubusercontent.com/u/48143394?v=4" width="100px;" alt=""/><br /><sub><b>smackhack</b></sub></a><br /><a href="#ideas-smackhack" title="Ideas, Planning, & Feedback">🤔</a> <a href="#example-smackhack" title="Examples">💡</a></td>
    <td align="center"><a href="http://bernardoamc.github.io/"><img src="https://avatars0.githubusercontent.com/u/428984?v=4" width="100px;" alt=""/><br /><sub><b>Bernardo Araujo</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=bernardoamc" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3Abernardoamc" title="Bug reports">🐛</a> <a href="#design-bernardoamc" title="Design">🎨</a></td>
    <td align="center"><a href="https://github.com/Isona"><img src="https://avatars2.githubusercontent.com/u/11759523?v=4" width="100px;" alt=""/><br /><sub><b>Izzy Whistlecroft</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/issues?q=author%3AIsona" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://imlonghao.com"><img src="https://avatars1.githubusercontent.com/u/4951333?v=4" width="100px;" alt=""/><br /><sub><b>imlonghao</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/issues?q=author%3Aimlonghao" title="Bug reports">🐛</a> <a href="#maintenance-imlonghao" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/royharoush"><img src="https://avatars3.githubusercontent.com/u/8113056?v=4" width="100px;" alt=""/><br /><sub><b>royharoush</b></sub></a><br /><a href="#ideas-royharoush" title="Ideas, Planning, & Feedback">🤔</a> <a href="#design-royharoush" title="Design">🎨</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/Atul9"><img src="https://avatars1.githubusercontent.com/u/3390330?v=4" width="100px;" alt=""/><br /><sub><b>Atul Bhosale</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=Atul9" title="Code">💻</a></td>
    <td align="center"><a href="https://tgotwig.me"><img src="https://avatars0.githubusercontent.com/u/30773779?v=4" width="100px;" alt=""/><br /><sub><b>Thomas Gotwig</b></sub></a><br /><a href="#platform-TGotwig" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/remigourdon"><img src="https://avatars3.githubusercontent.com/u/2874133?v=4" width="100px;" alt=""/><br /><sub><b>Rémi Gourdon</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=remigourdon" title="Documentation">📖</a> <a href="https://github.com/RustScan/RustScan/commits?author=remigourdon" title="Code">💻</a></td>
    <td align="center"><a href="https://cmnatic.co.uk"><img src="https://avatars3.githubusercontent.com/u/4163116?v=4" width="100px;" alt=""/><br /><sub><b>Ben (CMNatic)</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=cmnatic" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/commits?author=cmnatic" title="Documentation">📖</a> <a href="#design-cmnatic" title="Design">🎨</a></td>
    <td align="center"><a href="https://github.com/Ferryistaken"><img src="https://avatars3.githubusercontent.com/u/47927670?v=4" width="100px;" alt=""/><br /><sub><b>Alessandro Ferrari</b></sub></a><br /><a href="#content-Ferryistaken" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/Phenomite"><img src="https://avatars2.githubusercontent.com/u/8285537?v=4" width="100px;" alt=""/><br /><sub><b>Phenomite</b></sub></a><br /><a href="#content-Phenomite" title="Content">🖋</a></td>
    <td align="center"><a href="https://supersandro.de/"><img src="https://avatars2.githubusercontent.com/u/7258858?v=4" width="100px;" alt=""/><br /><sub><b>Sandro</b></sub></a><br /><a href="#content-SuperSandro2000" title="Content">🖋</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3ASuperSandro2000" title="Bug reports">🐛</a> <a href="https://github.com/RustScan/RustScan/commits?author=SuperSandro2000" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://swag.lgbt"><img src="https://avatars2.githubusercontent.com/u/25358963?v=4" width="100px;" alt=""/><br /><sub><b>Cass</b></sub></a><br /><a href="#platform-caass" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/RustScan/RustScan/commits?author=caass" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3Acaass" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/niklasmohrin"><img src="https://avatars0.githubusercontent.com/u/47574893?v=4" width="100px;" alt=""/><br /><sub><b>Niklas Mohrin</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=niklasmohrin" title="Documentation">📖</a> <a href="https://github.com/RustScan/RustScan/commits?author=niklasmohrin" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3Aniklasmohrin" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://liberapay.com/Artem4/"><img src="https://avatars0.githubusercontent.com/u/5614476?v=4" width="100px;" alt=""/><br /><sub><b>Artem Polishchuk</b></sub></a><br /><a href="#platform-tim77" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/buermarc"><img src="https://avatars2.githubusercontent.com/u/44375277?v=4" width="100px;" alt=""/><br /><sub><b>buermarc</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=buermarc" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/bergabman"><img src="https://avatars1.githubusercontent.com/u/44554109?v=4" width="100px;" alt=""/><br /><sub><b>bergabman</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=bergabman" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/issues?q=author%3Abergabman" title="Bug reports">🐛</a> <a href="#design-bergabman" title="Design">🎨</a></td>
    <td align="center"><a href="https://github.com/dmitris"><img src="https://avatars0.githubusercontent.com/u/31205?v=4" width="100px;" alt=""/><br /><sub><b>Dmitry Savintsev</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=dmitris" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/bofh69"><img src="https://avatars3.githubusercontent.com/u/1444315?v=4" width="100px;" alt=""/><br /><sub><b>Sebastian Andersson</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=bofh69" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/mattcorbin"><img src="https://avatars3.githubusercontent.com/u/6537765?v=4" width="100px;" alt=""/><br /><sub><b>Matt Corbin</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=mattcorbin" title="Code">💻</a></td>
    <td align="center"><a href="http://rootsploit.com"><img src="https://avatars2.githubusercontent.com/u/67270834?v=4" width="100px;" alt=""/><br /><sub><b>RootSploit</b></sub></a><br /><a href="#blog-rootsploit" title="Blogposts">📝</a></td>
    <td align="center"><a href="https://github.com/eiffel-fl"><img src="https://avatars2.githubusercontent.com/u/12171754?v=4" width="100px;" alt=""/><br /><sub><b>eiffel-fl</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=eiffel-fl" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/u5surf"><img src="https://avatars1.githubusercontent.com/u/14180225?v=4" width="100px;" alt=""/><br /><sub><b>Y.Horie</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=u5surf" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/okrplay"><img src="https://avatars3.githubusercontent.com/u/32576280?v=4" width="100px;" alt=""/><br /><sub><b>Oskar</b></sub></a><br /><a href="https://github.com/RustScan/RustScan/commits?author=okrplay" title="Code">💻</a> <a href="https://github.com/RustScan/RustScan/commits?author=okrplay" title="Tests">⚠️</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
