<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<br />
<div align="center">
  <a href="https://github.com/pedromiguelmiranda/another-rust-chat">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center"> Another rust chat</h3>

  <p align="center">
    TCP socket based chat built in Rust programming language
    <br />
    <a href="https://github.com/pedromiguelmiranda/another-rust-chat"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/pedromiguelmiranda/another-rust-chat">View Demo</a>
    ·
    <a href="https://github.com/pedromiguelmiranda/another-rust-chat/issues">Report Bug</a>
    ·
    <a href="https://github.com/pedromiguelmiranda/another-rust-chat/issues">Request Feature</a>
  </p>
</div>

![Product Name Screen Shot][product-screenshot]




I built this chat to improve my skills on Rust programming language. feel free to use it.

The server listens on 127.0.0.1:8080.

Each connected client is handled in a separate thread.

The clients vector contains all connected clients' streams and is wrapped in an Arc<Mutex<>> for thread safety. When a message is received from a client, it's broadcasted to all other connected clients.

When a client disconnects, its stream is removed from the clients vector.

Remember, this is still a simple example and doesn't handle all edge cases, such as errors during read/write operations 
or handling disconnected clients gracefully. In production-level code, you'd want to add more error handling and robustness features.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With


[![Rust][Rust-shield]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites


### Installation/execution

build and run
```
$ cargo build
$ cargo run
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap
- [ v ] basic server and client
- [ ] Change server concurrency pattern
- [ ] Add prompt to client
- [ ] create client dedicated structures 


See the [open issues](https://github.com/pedromiguelmiranda/another-rust-chat/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>




## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contact
Project Link: [https://github.com/pedromiguelmiranda/another-rust-chat](https://github.com/pedromiguelmiranda/another-rust-chat)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/pedromiguelmiranda/another-rust-chat?style=for-the-badge
[contributors-url]: https://github.com/pedromiguelmiranda/another-rust-chat/graphs/contributors
[forks-shield]:https://img.shields.io/github/forks/pedromiguelmiranda/another-rust-chat?style=for-the-badge
[forks-url]: https://github.com/pedromiguelmiranda/another-rust-chat/network/members
[stars-shield]: https://img.shields.io/github/stars/pedromiguelmiranda/another-rust-chat?style=for-the-badge
[stars-url]: https://github.com/pedromiguelmiranda/another-rust-chat/stargazers
[issues-shield]: https://img.shields.io/github/stars/pedromiguelmiranda/another-rust-chat?style=for-the-badge
[issues-url]: https://github.com/pedromiguelmiranda/another-rust-chat/issues
[license-shield]: https://img.shields.io/github/issues/pedromiguelmiranda/another-rust-chat?style=for-the-badge
[license-url]: https://github.com/pedromiguelmiranda/another-rust-chat/blob/master/LICENSE.txt
[Rust-shield]: https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[product-screenshot]: images/screenshot.png