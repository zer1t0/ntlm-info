# ntlm-info

[![Crates.io](https://img.shields.io/crates/v/ntlm-info)](https://crates.io/crates/ntlm-info)
[![Language Rust](https://img.shields.io/badge/Language-Rust-blue)](https://www.rust-lang.org/)

Retrieve the host information that is specified in the NTLM challenge.

This can be useful as a way to discover the names of the computers in an 
internal network, as an alternative to a reverse DNS query, but also to 
discover the name from the internal domain from hosts exposed to internet.


Currently it is possible to use the following application protocols to get 
an NTLM challenge:
- HTTP
- SMB


## SMB Usage

Quick example, to retrieve the names for the hosts in a local network you can do:
```shell
$ ntlm-info smb 192.168.100.0/24 -w 20

Target: 192.168.100.7
NbComputer: WS02-7
NbDomain: CONTOSO
DnsComputer: ws02-7.contoso.local
DnsDomain: contoso.local
Version: 6.1.7601
OS: Windows 7 | Windows Server 2008 R2

Target: 192.168.100.10
NbComputer: WS01-10
NbDomain: CONTOSO
DnsComputer: ws01-10.contoso.local
DnsDomain: contoso.local
Version: 10.0.19041
OS: Windows 10 | Windows Server 2019 | Windows Server 2016
```

As input for smb command, you can specify a...
- Hostname
- IP
- network CIDR

Moreover you can specify those in a file, in the parameters or stdin.

```shell
cat hosts.txt | ntlm-info smb
```

```shell
ntlm-info smb 192.168.100.10 192.168.100.7
```

```shell
ntlm-info smb 192.168.100.0/24
```


## HTTP Usage

Quick example, to retrieve info of an http endpoint:
```shell
$ ntlm-info http http://contoso.com/ 

Target: 192.168.100.10
NbComputer: WS01-10
NbDomain: CONTOSO
DnsComputer: ws01-10.contoso.local
DnsDomain: contoso.local
Version: 10.0.19041
OS: Windows 10 | Windows Server 2019 | Windows Server 2016
```

As input for http command, you can specify one or several URLs.

Moreover you can specify those in a file, in the parameters or stdin.

```shell
cat urls.txt | ntlm-info http
```

```shell
ntlm-info http http://contoso.com/ http://company.com/owa
```

## Installation

From crates:
```sh
cargo install ntlm-info
```

From repo:
```sh
cargo install --git https://github.com/Zer1t0/ntlm-info.git
```

To build it statically in Windows (Powershell):
```powershell
git clone https://github.com/Zer1t0/ntlm-info.git
cd ntlm-info/
$env:RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release
```


## Acknowledgments

This tool was inspired by [ntlm_challenger](https://github.com/domainicus/ntlm_challenger)

