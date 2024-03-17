# Minecraft server (made with Rust)
It is a project that eliminates the necessary hardware performance cost and optimizes the server from the bottom without the economic difficulty of purchasing expensive hosting servers to operate Minecraft servers

# Roadmap

- [x] server list ping
- [ ] entity metadata
- [ ] chunk, and palette
- [ ] packet encryption 
- [ ] packet compression
- [ ] mojang login request
- [ ] tick system 

## Why do I have to start this project and make something new
No Minecraft server existed that satisfies the requirements below.

## Project Requirements
- In Raspberry Pi Zero 2W (ARM Cortex A53 processor clocked at 1GHz and 512MB RAM) environments, high performance server applications are required to hold more than 30 users
- Different versions of clients connect to a server
- AntiCheat
- Various minigames

## my diagram 
![my_diagram](https://github.com/Bruce0203/minecraft-server-rs/assets/56539682/0f3c4cdb-71ec-4717-901a-90422650bacd)

---

# Discussion 

## Things that I need to think about more and make sure to decide 
- A client for testing shall be implemented.
- Pluggable selector pattern design structure or ECS (Entity Component System) structure

## things that have already been carefully decided in the project 
- Implement Minecraft Server as Rust
- Let's use the mio crate, without having to implement the socket selector kqueue/epoll directly 

---

# an appendix 

## Why did you choose Rust Language
- None of the servers originally made in the Java language had the performance I wanted.
- Java is slow. It takes as much as 7 nanoseconds to create a single buffer. (It was slower than Rust, which takes up to 1 nanosecond at its best performance.)
- The reason for choosing Rust over C/C++ is that Rust has a more powerful type system than Java/Kotlin.

## Why use the standard library's buffer Cursor<Vec<u8>> without using the BytesMut of bytes crate
[click here](https://github.com/Bruce0203/BytesMut_vs_Cursor_Vec_benchmark)

--

[![image](https://wiki.vg/resources/assets/licenses/cc-by-sa.png)](https://creativecommons.org/licenses/by-sa/3.0/)

