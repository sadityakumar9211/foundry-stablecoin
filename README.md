# Rusty-DNS
> This project is under development.

> Most recent functionality -> Created a stub resolver which takes domain input from user and queries the Google's Public DNS server (8.8.8.8) and shows the query and response packets to the console. Has the capability to parse and show various record types.

## Getting Started
1. Clone this repository
```zsh
git clone https://github.com/sadityakumar9211/rusty-dns
```

2. Switch to the `recursive-resolver` branch
```zsh
git checkout recursive-resolver
```

3. Install dependencies
```bash
cargo update
```

4. Build and Run: 
```bash
cargo build && cargo run
```
This will spin up a DNS server (recursive resolver) on 127.0.0.1:2053 which can be queried using `dig` tool to get the ip address response.

5. Split the terminal and query:
```bash
dig @127.0.0.1 -p 2053 <domain_name>
```

6. Response
You will see responses on both the terminals: 
- In the server terminal: 

<details>
  <summary>Check out the Output</summary>

```text
Received query: DnsQuestion { name: "www.reddit.com", qtype: A }
attempting lookup of A www.reddit.com with ns 198.41.0.4
Ok(
    DnsPacket {
        header: DnsHeader {
            id: 6666,
            recursion_desired: true,
            truncated_message: true,
            authoritative_answer: false,
            opcode: 0,
            response: true,
            rescode: NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,
            questions: 1,
            answers: 0,
            authoritative_entries: 13,
            resource_entries: 11,
        },
        questions: [
            DnsQuestion {
                name: "www.reddit.com",
                qtype: A,
            },
        ],
        answers: [],
        authorities: [
            NS {
                domain: "com",
                host: "e.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "b.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "j.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "m.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "i.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "f.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "a.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "g.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "h.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "l.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "k.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "c.gtld-servers.net",
                ttl: 172800,
            },
            NS {
                domain: "com",
                host: "d.gtld-servers.net",
                ttl: 172800,
            },
        ],
        resources: [
            A {
                domain: "e.gtld-servers.net",
                addr: 192.12.94.30,
                ttl: 172800,
            },
            AAAA {
                domain: "e.gtld-servers.net",
                addr: 2001:502:1ca1::30,
                ttl: 172800,
            },
            A {
                domain: "b.gtld-servers.net",
                addr: 192.33.14.30,
                ttl: 172800,
            },
            AAAA {
                domain: "b.gtld-servers.net",
                addr: 2001:503:231d::2:30,
                ttl: 172800,
            },
            A {
                domain: "j.gtld-servers.net",
                addr: 192.48.79.30,
                ttl: 172800,
            },
            AAAA {
                domain: "j.gtld-servers.net",
                addr: 2001:502:7094::30,
                ttl: 172800,
            },
            A {
                domain: "m.gtld-servers.net",
                addr: 192.55.83.30,
                ttl: 172800,
            },
            AAAA {
                domain: "m.gtld-servers.net",
                addr: 2001:501:b1f9::30,
                ttl: 172800,
            },
            A {
                domain: "i.gtld-servers.net",
                addr: 192.43.172.30,
                ttl: 172800,
            },
            AAAA {
                domain: "i.gtld-servers.net",
                addr: 2001:503:39c1::30,
                ttl: 172800,
            },
            A {
                domain: "f.gtld-servers.net",
                addr: 192.35.51.30,
                ttl: 172800,
            },
        ],
    },
)
attempting lookup of A www.reddit.com with ns 192.12.94.30
Ok(
    DnsPacket {
        header: DnsHeader {
            id: 6666,
            recursion_desired: true,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: true,
            rescode: NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,
            questions: 1,
            answers: 0,
            authoritative_entries: 4,
            resource_entries: 1,
        },
        questions: [
            DnsQuestion {
                name: "www.reddit.com",
                qtype: A,
            },
        ],
        answers: [],
        authorities: [
            NS {
                domain: "reddit.com",
                host: "ns-557.awsdns-05.net",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-378.awsdns-47.com",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-1029.awsdns-00.org",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-1887.awsdns-43.co.uk",
                ttl: 172800,
            },
        ],
        resources: [
            A {
                domain: "ns-378.awsdns-47.com",
                addr: 205.251.193.122,
                ttl: 172800,
            },
        ],
    },
)
attempting lookup of A www.reddit.com with ns 205.251.193.122
Ok(
    DnsPacket {
        header: DnsHeader {
            id: 6666,
            recursion_desired: true,
            truncated_message: false,
            authoritative_answer: true,
            opcode: 0,
            response: true,
            rescode: NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,
            questions: 1,
            answers: 1,
            authoritative_entries: 4,
            resource_entries: 0,
        },
        questions: [
            DnsQuestion {
                name: "www.reddit.com",
                qtype: A,
            },
        ],
        answers: [
            CNAME {
                domain: "www.reddit.com",
                host: "reddit.map.fastly.net",
                ttl: 10800,
            },
        ],
        authorities: [
            NS {
                domain: "reddit.com",
                host: "ns-1029.awsdns-00.org",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-1887.awsdns-43.co.uk",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-378.awsdns-47.com",
                ttl: 172800,
            },
            NS {
                domain: "reddit.com",
                host: "ns-557.awsdns-05.net",
                ttl: 172800,
            },
        ],
        resources: [],
    },
)
Answer: CNAME { domain: "www.reddit.com", host: "reddit.map.fastly.net", ttl: 10800 }
Authority: NS { domain: "reddit.com", host: "ns-1029.awsdns-00.org", ttl: 172800 }
Authority: NS { domain: "reddit.com", host: "ns-1887.awsdns-43.co.uk", ttl: 172800 }
Authority: NS { domain: "reddit.com", host: "ns-378.awsdns-47.com", ttl: 172800 }
Authority: NS { domain: "reddit.com", host: "ns-557.awsdns-05.net", ttl: 172800 }
```
</details>
<br>

- In the dig terminal:
<details>
  <summary>Check out the Output</summary>

```text
; <<>> DiG 9.10.6 <<>> @127.0.0.1 -p 2053 www.reddit.com
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 35824
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 4, ADDITIONAL: 0

;; QUESTION SECTION:
;www.reddit.com.                        IN      A

;; ANSWER SECTION:
www.reddit.com.         10800   IN      CNAME   reddit.map.fastly.net.

;; AUTHORITY SECTION:
reddit.com.             172800  IN      NS      ns-1029.awsdns-00.org.
reddit.com.             172800  IN      NS      ns-1887.awsdns-43.co.uk.
reddit.com.             172800  IN      NS      ns-378.awsdns-47.com.
reddit.com.             172800  IN      NS      ns-557.awsdns-05.net.

;; Query time: 436 msec
;; SERVER: 127.0.0.1#2053(127.0.0.1)
;; WHEN: Mon Aug 28 08:37:37 IST 2023
;; MSG SIZE  rcvd: 261
```
</details>

When you run `cargo run`, it spins up a server which is listening for UDP packets on port 2053 on 127.0.0.1 (localhost). When you query for `www.reddit.com` using `dig` it catches the packet from and retransmits it to the one of the 13 logical root nameserver. It gets the response for the TLD nameservers handling the `.com` domain. It again queries one of the TLD nameservers and in response gets the IP addresses of authoritative nameservers which are handling the `reddit.com` zone. It further queries one of these nameservers and finally gets a response with `answers` section filled with the IP address of `www.reddit.com` domain. Finally, it returns this result to `dig` by encoding this result in a DNS packet. `dig` parses this packet and shows the result in the console.


## Developer Notes
- This will consist of 5 phases. Currently Developing under Phase 3.
- With this project, I will be writing blogs on each phase of this project.
- The blogs will be available at my [blog website](https://saditya9211.hashnode.dev/series/rusty-dns).

## Points to Ponder
Q. Why we need to create UDP socket to send a UDP packet when it is connectionless?


While UDP is a connectionless protocol, creating a UDP socket is essential to facilitate the sending and receiving of UDP packets in a structured and controlled manner. Here's why you need to create a UDP socket even though UDP is connectionless:

1. **Abstraction**: Sockets provide an abstraction over the underlying network communication protocols. They allow your application to interact with the network in a consistent way, regardless of whether the protocol is connection-oriented (like TCP) or connectionless (like UDP).

2. **Addressing and Port Binding**: When you create a UDP socket, you specify the local address and port from which you want to send or receive UDP packets. This allows your application to specify where the data should be sent or received on the network.

3. **Error Handling**: While UDP itself doesn't provide guaranteed delivery or error correction, your application can implement error handling and recovery logic. A UDP socket allows you to detect transmission errors and lost packets and decide how to respond.

4. **Buffering**: Sockets provide buffering mechanisms to manage incoming and outgoing data. This can help with handling bursts of data and provide a level of flow control even in a connectionless protocol like UDP.

5. **Multicasting and Broadcasting**: UDP sockets are used for sending multicast and broadcast packets. These features are not built into the UDP protocol directly but are managed through the socket API.

6. **Application Layer Logic**: Sockets provide a way to interact with the network at the application layer. While the lower layers handle packet routing and delivery, the application needs to make decisions about how to interpret and handle the data received or sent. The socket API provides this interface.

7. **Interface to OS Network Stack**: Sockets act as an interface to the operating system's network stack. They allow you to make use of the networking capabilities of the operating system in a platform-independent way.

In summary, creating a UDP socket provides your application with a structured way to interact with the UDP protocol and the underlying network infrastructure. While UDP itself is connectionless and lacks features like guaranteed delivery, sockets offer the necessary control and abstraction for sending and receiving UDP packets within your application.

## Phases
1. **The DNS Protocol** - Write a DNS packet parser and learn about the intricacies of domain name encoding using labels and about other fields of a DNS packet. ✅
2. **Building a stub resolver**: Create a stub resolver which quries a domain from Google's public DNS resolver (`8.8.8.8`). ✅
3. **Adding various Record Types**: Added various record types. ✅
4. **DNS server Implementation**: Created a DNS server for listening to `dig` and querying `8.8.8.8` and responding back to `dig` with response DNS packet. ✅
5. **Implementing Recursive Resolvers**: Created a recursive resolver which queries the DNS infrastructure recursively to get the IP address of a domain. ✅




