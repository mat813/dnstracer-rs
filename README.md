
# dnstracer

`dnstracer` is a Rust implementation of the DNS tracing tool `dnstracer`. It follows the chain of DNS servers that resolve a domain name. It shows the delegation path from the root DNS servers to the authoritative servers of a domain.

## Features

- Traces the DNS delegation path of a domain name.
- Supports IPv4 and IPv6.
- Supports the selection of the DNS query type.
- Follows NS delegations and CNAME chains.
- Shows the DNS servers and the records that they return.

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.93 or later
- Cargo, the Rust package manager

### Install dnstracer

Install `dnstracer` with `cargo`:

```bash
cargo install dnstracer
```

### Run

Run `dnstracer` with a domain name:

```bash
dnstracer [OPTIONS] <domain>
```

### Example

```bash
dnstracer example.com
```

This command traces the DNS delegation of `example.com`. It shows the DNS servers on the path.

## Usage

```bash
dnstracer [OPTIONS] <domain>
```

### Options

- `-c`, `--no-positive-cache`
  - Disable the cache of positive responses (default: enabled)
- `-C`, `--negative-cache`
  - Enable the cache of negative responses (default: disabled)
- `-e`, `--no-edns0`
  - Disable EDNS0 (default: enabled)
- `-o`, `--overview`
  - Show an overview of the received answers (default: disabled)
- `-m`, `--max-queries <MAX_QUERIES>`
  - The maximum number of DNS queries for the whole trace [default: 200]
- `-q`, `--query-type <QUERY_TYPE>`
  - The record type to query (A, AAAA, NS, ...) [default: A]
- `-r`, `--retries <RETRIES>`
  - The number of retries for each DNS request [default: 3]
- `-s`, `--server <SERVER>`
  - Start the query at the given DNS server. For an IP address, use that address. For a hostname, use all the IP addresses of that hostname. For ".", use all the root servers [default: a.root-servers.net]
- `-t`, `--timeout <TIMEOUT>`
  - The time limit, in seconds, for each try [default: 5]
- `-S`, `--source-address <SOURCE_ADDRESS>`
  - The source address of the DNS queries
- `-6`, `--ipv6`
  - Use only IPv6 for DNS queries
- `-4`, `--ipv4`
  - Use only IPv4 for DNS queries
- `-T`, `--tcp`
  - Use only TCP for DNS queries
- `-h`, `--help`
  - Print help
- `-V`, `--version`
  - Print version

## Output

`dnstracer` shows these items for each DNS server in the delegation chain:

- The IP address and the name of the server.
- If the response is authoritative.
- The DNS records that the server returns.

### Sample Output

```text
$ dnstracer www.example.com -o              
Tracing to www.example.com[A] via A.ROOT-SERVERS.NET. (198.41.0.4), maximum of 3 retries
A.ROOT-SERVERS.NET. [.] (198.41.0.4)
 |\___ a.gtld-servers.net. [com] (192.5.6.30)
 |      |\___ ns1.example.com. [example.com] (192.0.2.1) found authoritative answer
 |      |\___ ns2.example.com. [example.com] (198.51.100.1) found authoritative answer
 |       \___ ns2.example.com. [example.com] (2001:db8::1) found authoritative answer
 |\___ a.gtld-servers.net. [com] (2001:503:a83e::2:30)
 |      |\___ ns1.example.com. [example.com] (192.0.2.1) (cached)
 |      |\___ ns2.example.com. [example.com] (198.51.100.1) (cached)
 |       \___ ns2.example.com. [example.com] (2001:db8::1) (cached)
 |\___ b.gtld-servers.net. [com] (192.33.14.30)
 ...
ns1.example.com. (192.0.2.1) 	www.example.com. 86400 IN A 203.0.113.1
ns2.example.com. (198.51.100.1) 	www.example.com. 86400 IN A 203.0.113.1
ns2.example.com. (2001:db8::1) 	www.example.com. 86400 IN A 203.0.113.1
```

## Contributing

1. Fork the repository.
2. Create a branch: `git checkout -b feature/your-feature`.
3. Commit your changes: `git commit -am 'Add some feature'`.
4. Push the branch: `git push origin feature/your-feature`.
5. Create a pull request.

## License

The MIT License applies to this project. See the [LICENSE](LICENSE) file.
