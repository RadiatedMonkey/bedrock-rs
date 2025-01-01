# bedrock-rs

**_Universal Toolkit for Minecraft Bedrock Edition in Rust_**

**bedrock-rs** is a comprehensive and user-friendly library written in Rust, designed to provide a universal solution for working with Minecraft Bedrock Edition. This project offers:  

- **Standards:** Adhering to best practices and conventions.  
- **Common Implementations:** Reusable components for various Minecraft Bedrock needs.  
- **Easy-to-Use API:** Streamlined interfaces to make development efficient and enjoyable.  

Join our growing community on Discord to learn more about the project’s future, seek support, or collaborate with others:  
**[Join our Discord](https://discord.com/invite/VCVcrvt3JC)**  

---

## Crates

To maintain modularity and scalability, **bedrock-rs** is divided into multiple crates. Each crate focuses on a specific functionality, making it easier to use and manage. All crates are accessible through the primary `bedrockrs` crate. Additionally, the library offers a variety of optional features you can enable to suit your needs.

### Crate Breakdown:

- [`bedrockrs::shared`](crates/shared)  
    - [X] Shared data types used across other crates.  
    - [X] Support for deriving macros defined in other modules.  

- [`bedrockrs::form`](crates/form)  
    - [X] Implementation of the JSON form format used by Minecraft Bedrock Edition.  

- [`bedrockrs::addon`](crates/addon)  
    - [X] Datatypes for defining Minecraft Addon structures.  
    - [X] Serialization and deserialization support for Addons.  
    - [X] A programmatic approach to creating Addons easily.  

- [`bedrockrs::proto`](crates/proto)  
    - [X] Complete implementation of the Minecraft Bedrock protocol.  
    - [X] Support for both server-side and client-side operations.  
    - [X] Multi-protocol compatibility for handling multiple versions seamlessly.  

- [`bedrockrs::level`](crates/level)  
    - [X] Data structures for managing Minecraft Bedrock levels.  
    - [X] Implementation of Bedrock’s level format using Rust’s LevelDB.  

- [`bedrockrs::server`](/crates/server)  
    - [X] A lightweight foundation for Minecraft Bedrock server software.  
    - [X] Built-in support for multi-protocol handling.  
    - [X] Asynchronous architecture powered by Tokio.  

---

## Features

- **Modular Architecture:** Enable only the features you need for your project.  
- **Multi-Protocol Support:** Work with different protocol versions effortlessly.  
- **Cross-Platform Compatibility:** Designed to work seamlessly across platforms.  
- **Lightweight and Efficient:** Built with Rust’s performance and safety features.  

---

## Getting Started

To use **bedrock-rs** in your Rust project, add the following to your `Cargo.toml`:  
```toml
[dependencies]
bedrockrs = { git = "https://github.com/bedrock-crustaceans/bedrock-rs.git", features = ["full"] }
```

Refer to the individual crate documentation for details on specific modules and features.

We also plan to release bedrock-rs on [crates.io](https://crates.io) in the future.

---

## Contributors  

A huge thank you to all the amazing individuals who have contributed to **bedrock-rs**! Your time, effort, and expertise are what make this project possible.  

[![Contributors](https://contrib.rocks/image?repo=bedrock-crustaceans/bedrock-rs)](https://github.com/bedrock-crustaceans/bedrock-rs/graphs/contributors)

Whether it’s fixing bugs, implementing features, or providing feedback, every contribution helps shape the future of this library. We appreciate each and every one of you!  

Want to join this incredible group? Check out our Contributing Guide and make your mark on the project.  

---

## Contributing

We welcome contributions of all kinds! Whether you're fixing bugs, adding new features, or proposing entirely new modules, your efforts are highly appreciated. Here’s how you can get involved:  

1. **Get Familiar with the Codebase:** Explore the existing modules and documentation.  
2. **Pick an Issue:** Check out the repository’s issue tracker for tasks you can work on.  
3. **Add Your Touch:** Feel free to innovate and bring new ideas to the table.  

For guidance or collaboration, feel free to connect with the community on Discord.  

If you find **bedrock-rs** helpful, don’t forget to give the repository a ⭐ on GitHub.  

---

## License

**bedrock-rs** is open-source software licensed under the [Apache-2.0 License](LICENSE).  
