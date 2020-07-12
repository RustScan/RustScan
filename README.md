![Rust Scan](pictures/rustscan.png)

# RustScan
1. Find ports quickly using Rust.
2. Automatically runs `nmap -A -sV -p $ports -vvv` on those ports and IP addresses
3. Profit???

## Explanation
Nmap is the only good portscanner for CTFs. RustScan knows this, and does not try to replace it.

However, in a CTF, nmap scanning all 65k ports is **very** slow.

RustScan **compliements** Nmap. RustScan scns all 65k ports with extreme speed and concurrency, and then it pipes those ports into Nmap.

### Why not Massscan or copy and paste?
It takes time to physically type out the port numbers into Nmap from masscan. 

Masscan probably wouldn't add this feature, as they are looking to be a competitor to Nmap.

RustScan will execute Nmap the second it finds all the open ports. RustScan only wants to compliement Nmap, not beat it.


TL;DR RustScan developers throw away their ego, and understand that no tool will ever come close to Nmap and instead chooses to compliement Nmap.

# RustScan vs Nmap vs MassScan

| **Name**                                                                                   | RustScan | Nmap | Masscan |
| ------------------------------------------------------------------------------------------ | -------- | ---- | ------- |
| Fast                                                                                       | Y        | N    | Y       |
| Actually useful                                                                            | N        | Y    | N       |
| Realises it's not useful, and pipes the only useful data into the only useful port scanner | Y        | N    | N       |

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