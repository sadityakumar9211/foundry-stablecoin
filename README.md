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

Now we should close `netcat` process listening at port `1053` in the first terminal and try sending the captured DNS query to one of the public DNS servers. For this we can use the Google's public DNS server (`8.8.8.8`). So, after killing the `netcat` process which was listening at port `1053`, we send the DNS query packet to `8.8.8.8` by:
```zsh
nc -u 8.8.8.8 53 < packets/query_packet.txt > packets/response_packet.txt
```
What we are doing above is, we're sending the contents of `packets/query_packet.txt` to port `53`(default port for DNS protocol) of `8.8.8.8` server and we are redirecting the response received to `packet/response_packet.txt` file on disk.

5. Send this captured Query Packet to any public DNS server (Google - 8.8.8.8) and capture the DNS response into a file `packets/response_packet.txt`

7. Now run
```zsh
$ cargo build
$ cargo run
```

### Developer Notes:-
- This will consist of 5 phases. Currently Developing under Phase 1.
- With this project, I will be writing blogs on each phase of this project.
- The blogs will be available at my blog website: https://saditya9211.hashnode.dev


The Five phases of this project are:-
1. The DNS Protocol - Learning about DNS protocol and various techniques used to make it efficient.
2. Building a stub resolver
3. Adding various Record Types
4. Final DNS server Implementation
5. Implementing Recursive Resolvers
