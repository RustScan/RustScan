<p align="center">
<img src="pictures/rustscan.png"><br>
Find all open ports <b>fast</b> with Rustscan, automatically pipe them into Nmap. Built with Rust. 
</p>
<hr>

| General Linux | Cargo         | Arch         | 
| ------------- | ------------- | ------------ |
| img1          | img2          | img3         |
| Binaries      | Cargo install | yay rustscan |

<hr>

# 🤔 What is this?
If you are a competitive CTF player and often find yourself running masscan / a basic nmap scan before running a more comprehensive scan, this tool is for you.
1. Find ports quickly using Rustscan. 
2. Automatically runs `nmap -A -sV -p $ports -vvv` on those ports.
3. Profit???

Rustscans **only** job is to reduce the friction between finding open ports and inputting them into nmap.

# RustScan vs Nmap vs MassScan

| **Name**                                                                                   | RustScan | Nmap | Masscan |
| ------------------------------------------------------------------------------------------ | -------- | ---- | ------- |
| Fast                                                                                       | ✅        | ❌    | ✅       |
| Actually useful                                                                            | ❌        | ✅    | ❌       |
| Realises it's not useful, and pipes the only useful data into the only useful port scanner | ✅        | ❌    | ❌       |


## FAQ
> I think this would be a great port scanner on its own without Nmap!
> 
No. If you want a fast port scanner, use Masscan.
> I have this great idea for a script to get information on ports / hosts

Great. Contribute it to Nmap! :D
> Not everyone has nmap installed....

If you're a pentester, then yes, you have Nmap installed. 

> I want to contribute!

The only contributions RustScan are accecpting is:
* User Experience
* Making port scanning faster

# Ideas

- [ ] Do popular ports first (80, 445, 8080, 21, 22)
- [ ] Option for top 1k ports
- [ ] Affter 1k ports print the nmap command to scan the first 1k so user can run while rustscanner finishes
- [ ] Tar pit prevention (look for different sized payloads)
