# Rusty-DNS
> This project is under development.

> Most recent functionality -> DNS query/response packets can be parsed from disk for reading.

## Getting Started
1. Clone this repository
```zsh
git clone https://github.com/sadityakumar9211/rusty-dns
```

3. Create and capture a DNS Query Packet into a file `packets/query_packet.txt` on local disk
We will be achieving this using `netcat` command line tool and `dig` (Domain Information Gropper).
- Split the teminal into two. One for sending the packet and one for receiving.
- In the first terminal we will be listening to port `1053` of localhost and redirected any received data to `packets/query_packet.txt`.
```zsh
nc -u -l 1053 > packets/query_packet.txt
```
- In the second split terminal, we will send a DNS query packet to port `1053` of loopback address i.e. `127.0.0.1` using `dig` command line tool.
```zsh
dig +retry=0 -p 1053 @127.0.0.1 +noedns reddit.com
```
After few moments, the second terminal will show timeout, this is expected as the initiated query did not receive any response. However, our intiated query packet is captured by `netcat` and stored in `packet/query_packet.txt` file on disk.

Now we should close `netcat` process listening at port `1053` in the first terminal and try sending the captured DNS query to one of the public DNS servers.

5. Send this captured Query Packet to any public DNS server (Google - `8.8.8.8`) and capture the DNS response into a file `packets/response_packet.txt`.

```zsh
nc -u 8.8.8.8 53 < packets/query_packet.txt > packets/response_packet.txt
```
What we are doing above is, we're sending the contents of `packets/query_packet.txt` to port `53`(default port for DNS protocol) of `8.8.8.8` server and we are redirecting the response received to `packet/response_packet.txt` file on disk.


  
  


6. Now run
```zsh
$ cargo build
$ cargo run
```

You will get a response similar to this:
<details>
  <summary>Check out the Output</summary>

```text
The contents of packet header is:-
DnsHeader {
    id: 53919,
    recursion_desired: true,
    truncated_message: false,
    authoritative_answer: false,
    opcode: 0,
    response: true,
    rescode: NOERROR,
    checking_disabled: false,
    authed_data: false,
    z: false,
    recursion_available: true,
    questions: 1,
    answers: 4,
    authoritative_entries: 0,
    resource_entries: 0,
}


The contents of Question section is:-
DnsQuestion {
    name: "reddit.com",
    qtype: A,
}


The contents of Answer section is:-
A {
    domain: "reddit.com",
    addr: 151.101.193.140,
    ttl: 72,
}
A {
    domain: "reddit.com",
    addr: 151.101.65.140,
    ttl: 72,
}
A {
    domain: "reddit.com",
    addr: 151.101.129.140,
    ttl: 72,
}
A {
    domain: "reddit.com",
    addr: 151.101.1.140,
    ttl: 72,
}


The contents of Authority section is:-


The contents of Additional section is:-

```
</details>

When you run `cargo run`, the code basically parses each field of the response DNS packet (`packets/response_packet.txt`) and prints the contents of each of those fields. 

## Developer Notes:-
- This will consist of 5 phases. Currently Developing under Phase 2.
- With this project, I will be writing blogs on each phase of this project.
- The blogs will be available at my blog website: https://saditya9211.hashnode.dev/rusty-dns


The Five phases of this project are:-
1. The DNS Protocol - Write a DNS packet parser and learn about the intricacies of domain name encoding using labels and abou tother fields of a DNS packet.

2. Building a stub resolver - 
3. Adding various Record Types
4. Final DNS server Implementation
5. Implementing Recursive Resolvers
