# RustScan
An port scanner in Rust.

1. Find ports quickly in Rust.
2. Automatically runs `nmap -A -sV -p $ports -vvv` or whatever command is in settings.yml
3. Profit???

# Ideas

- [ ] Do popular ports first (80, 445, 8080, 21, 22)
- [ ] Option for top 1k ports
- [ ] Affter 1k ports print the nmap command to scan the first 1k so user can run while rustscanner finishes
- [ ] Tar pit prevention (look for different sized payloads)