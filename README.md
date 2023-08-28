# Rusty-DNS
> This project is under development.

> Most recent functionality -> Created a stub resolver which takes domain input from user and queries the Google's Public DNS server (8.8.8.8) and shows the query and response packets to the console. Has the capability to parse and show various record types.

## Getting Started
1. Clone this repository
```zsh
git clone https://github.com/sadityakumar9211/rusty-dns
```

2. Switch to the `more-record-types` branch
```zsh
git checkout our-own-server
```

3. Install dependencies
```bash
cargo update
```

4. Build and Run: 
```bash
cargo build && cargo run
```
This will spin up a DNS server on 127.0.0.1:2053 which can be queried using `dig` tool to get the ip address response.

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
DnsPacket {
    header: DnsHeader {
        id: 7666,
        recursion_desired: true,
        truncated_message: false,
        authoritative_answer: false,
        opcode: 0,
        response: false,
        rescode: NOERROR,
        checking_disabled: false,
        authed_data: true,
        z: false,
        recursion_available: false,
        questions: 1,
        answers: 0,
        authoritative_entries: 0,
        resource_entries: 1,
    },
    questions: [
        DnsQuestion {
            name: "reddit.com",
            qtype: A,
        },
    ],
    answers: [],
    authorities: [],
    resources: [
        UNKNOWN {
            domain: "",
            qtype: 41,
            data_len: 0,
            ttl: 0,
        },
    ],
}
Received query: DnsQuestion { name: "reddit.com", qtype: A }
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
            recursion_available: true,
            questions: 1,
            answers: 4,
            authoritative_entries: 0,
            resource_entries: 0,
        },
        questions: [
            DnsQuestion {
                name: "reddit.com",
                qtype: A,
            },
        ],
        answers: [
            A {
                domain: "reddit.com",
                addr: 151.101.129.140,
                ttl: 184,
            },
            A {
                domain: "reddit.com",
                addr: 151.101.1.140,
                ttl: 184,
            },
            A {
                domain: "reddit.com",
                addr: 151.101.193.140,
                ttl: 184,
            },
            A {
                domain: "reddit.com",
                addr: 151.101.65.140,
                ttl: 184,
            },
        ],
        authorities: [],
        resources: [],
    },
)
Answer: A { domain: "reddit.com", addr: 151.101.129.140, ttl: 184 }
Answer: A { domain: "reddit.com", addr: 151.101.1.140, ttl: 184 }
Answer: A { domain: "reddit.com", addr: 151.101.193.140, ttl: 184 }
Answer: A { domain: "reddit.com", addr: 151.101.65.140, ttl: 184 }
```
</details>
<br>

- In the dig terminal:
<details>
  <summary>Check out the Output</summary>

```text
; <<>> DiG 9.10.6 <<>> @127.0.0.1 -p 2053 reddit.com
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 7666
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;reddit.com.                    IN      A

;; ANSWER SECTION:
reddit.com.             184     IN      A       151.101.129.140
reddit.com.             184     IN      A       151.101.1.140
reddit.com.             184     IN      A       151.101.193.140
reddit.com.             184     IN      A       151.101.65.140

;; Query time: 145 msec
;; SERVER: 127.0.0.1#2053(127.0.0.1)
;; WHEN: Mon Aug 28 07:44:48 IST 2023
;; MSG SIZE  rcvd: 132

```
</details>

When you run `cargo run`, it spins up a server which is listening for UDP packets on port 2053 on 127.0.0.1 (localhost). When you query for `reddit.com` using `dig` it catches the packet from and retransmits it to the Google's public DNS resolver. This DNS resolver will query the DNS infrastructure recursively and finally respond with answers. The server will catch this packet and respond it to the source (which was `dig` tool itself).


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
5. **Implementing Recursive Resolvers**: 


