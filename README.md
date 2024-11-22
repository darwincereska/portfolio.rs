
# Portfolio Website in Rocket.rs 🚀

A modern, fast portfolio website built with Rust using the Rocket web framework.

## Features

- Fast and efficient server-side rendering
- Clean and minimalist design
- Built with Rust's safety and performance guarantees
- Rocket.rs framework for robust routing and handling
- Responsive design for all devices

## Technologies Used

- Rust
- Rocket.rs
- HTML/CSS
- HTMX for  client-side rendering
## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository

git clone https://github.com/yourusername/portfolio-rs.git
cd portfolio-rs


2. Build the project

cargo build --release


3. Run the server

cargo run


The website will be available at `http://localhost:8000`

## Project Structure


```├── .env
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── Rocket.toml
├── src
│   ├── main.rs
│   ├── pages
│   │   ├── 404.html
│   │   ├── about.html
│   │   ├── home.html
│   │   └── index.html
│   └── static
│       ├── 404
│       │   ├── script.js
│       │   └── styles.css
│       ├── favicon.ico
│       ├── htmx.min.js
│       ├── projects.json
│       └── tailwind.css
└── tailwind.config.js```

## Configuration

The application can be configured through the `Rocket.toml` file. See Rocket's [configuration guide](https://rocket.rs/v0.5/guide/configuration/) for more details.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact
For any questions or inquiries, please contact [darwin@durrstudios.dev](mailto:darwin@durrstudios.dev).
Project Link: [https://github.com/yourusername/portfolio-rs](https://github.com/darwincereska/portfolio-rs)
