# Rusty-DNS
> This project is under development.

> Most recent functionality -> Created a stub resolver which takes domain input from user and queries the Google's Public DNS server (8.8.8.8) and shows the query and response packets to the console.

## Getting Started
1. Clone this repository
```zsh
git clone https://github.com/sadityakumar9211/rusty-dns
```

2. Switch to the `more-record-types` branch
```zsh
git checkout more-record-types
```

3. Install dependencies
```bash
cargo update
```

4. Build and Run: 
```bash
cargo build && cargo run
```

5. Provide the domain you want to query IP address for: 
```bash
saditya rusty-dns % cargo build && cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/dnsfun`
Enter a domain name(example.com): 
www.reddit.com
```

6. You will get a response similar to this:
<details>
  <summary>Check out the Output</summary>

```text
########## DNS Query Packet ##########
DnsPacket {
    header: DnsHeader {
        id: 6666,
        recursion_desired: true,
        truncated_message: false,
        authoritative_answer: false,
        opcode: 0,
        response: false,
        rescode: NOERROR,
        checking_disabled: false,
        authed_data: false,
        z: false,
        recursion_available: false,
        questions: 1,
        answers: 0,
        authoritative_entries: 0,
        resource_entries: 0,
    },
    questions: [
        DnsQuestion {
            name: "www.reddit.com",
            qtype: A,
        },
    ],
    answers: [],
    authorities: [],
    resources: [],
}




########## DNS Response Packet ##########
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
        answers: 5,
        authoritative_entries: 0,
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
            ttl: 9834,
        },
        A {
            domain: "reddit.map.fastly.net",
            addr: 151.101.1.140,
            ttl: 25,
        },
        A {
            domain: "reddit.map.fastly.net",
            addr: 151.101.65.140,
            ttl: 25,
        },
        A {
            domain: "reddit.map.fastly.net",
            addr: 151.101.129.140,
            ttl: 25,
        },
        A {
            domain: "reddit.map.fastly.net",
            addr: 151.101.193.140,
            ttl: 25,
        },
    ],
    authorities: [],
    resources: [],
}
```
</details>

When you run `cargo run`, the code creates a DNS packet consisting of various sections viz. header, questions, answers, authorities, resources and it sends a UDP packet to one of the Google's public DNS resolver(server). The resolver queries for the IP address from the underlying DNS infrastructure and responds with the response packet containing the IP address of queries domain in the `answers` field of DNS response packet. 


## Developer Notes
- This will consist of 5 phases. Currently Developing under Phase 3.
- With this project, I will be writing blogs on each phase of this project.
- The blogs will be available at my [blog website](https://saditya9211.hashnode.dev/series/rusty-dns).

## Points to Ponder
Q. Why we need to create UDP socket to send a UDP packet when it is connectionless?

<<<<<<< Updated upstream

While UDP is a connectionless protocol, creating a UDP socket is essential to facilitate the sending and receiving of UDP packets in a structured and controlled manner. Here's why you need to create a UDP socket even though UDP is connectionless:

1. **Abstraction**: Sockets provide an abstraction over the underlying network communication protocols. They allow your application to interact with the network in a consistent way, regardless of whether the protocol is connection-oriented (like TCP) or connectionless (like UDP).

2. **Addressing and Port Binding**: When you create a UDP socket, you specify the local address and port from which you want to send or receive UDP packets. This allows your application to specify where the data should be sent or received on the network.

3. **Error Handling**: While UDP itself doesn't provide guaranteed delivery or error correction, your application can implement error handling and recovery logic. A UDP socket allows you to detect transmission errors and lost packets and decide how to respond.

4. **Buffering**: Sockets provide buffering mechanisms to manage incoming and outgoing data. This can help with handling bursts of data and provide a level of flow control even in a connectionless protocol like UDP.

5. **Multicasting and Broadcasting**: UDP sockets are used for sending multicast and broadcast packets. These features are not built into the UDP protocol directly but are managed through the socket API.

6. **Application Layer Logic**: Sockets provide a way to interact with the network at the application layer. While the lower layers handle packet routing and delivery, the application needs to make decisions about how to interpret and handle the data received or sent. The socket API provides this interface.

7. **Interface to OS Network Stack**: Sockets act as an interface to the operating system's network stack. They allow you to make use of the networking capabilities of the operating system in a platform-independent way.

In summary, creating a UDP socket provides your application with a structured way to interact with the UDP protocol and the underlying network infrastructure. While UDP itself is connectionless and lacks features like guaranteed delivery, sockets offer the necessary control and abstraction for sending and receiving UDP packets within your application.



=======
>>>>>>> Stashed changes
## Phases
1. **The DNS Protocol** - Write a DNS packet parser and learn about the intricacies of domain name encoding using labels and about other fields of a DNS packet. ✅

2. **Building a stub resolver**: Create a stub resolver which quries a domain from Google's public DNS resolver (`8.8.8.8`). ✅
<<<<<<< Updated upstream
3. **Adding various Record Types**: Active Development
4. **Final DNS server Implementation**
5. **Implementing Recursive Resolvers**
=======
3. **Adding various Record Types**: Added various record types including A, AAAA, CNAME, NS, and MX. ✅
4. **Final DNS server Implementation** 
5. **Implementing Recursive Resolvers**
>>>>>>> Stashed changes
