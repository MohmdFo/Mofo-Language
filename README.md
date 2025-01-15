# Mofo Language

Mofo Language is a static-typed, interpreted programming language inspired by Python. It offers Python-like syntax with built-in support for `printf` and static typing. Variables must be declared with their type during assignment.

## Features
- **Static Typing**: Variables are declared with a type (e.g., `name: str = "value"`).
- **Python-like Syntax**: Easy to read and write.
- **Built-in `printf` Function**: Print output to the terminal.
- **Variable Declaration and Usage**: Supports types like `str`.

## Installation

### From Installer
1. Download the latest `setup.exe` from the [Releases](#) section.
2. Run the installer and follow the instructions.
3. After installation, restart your terminal or command prompt.
4. Verify installation:
   ```bash
   mofo --version
   ```
   Expected output:
   ```plaintext
   Mofo Language v0.1.0
   ```

### From Source
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/mofo-language.git
   cd mofo-language
   ```
2. Build and install the language:
   ```bash
   cargo install --path .
   ```
3. Verify installation:
   ```bash
   mofo --version
   ```

## Usage

### Running a `.mofo` File
1. Create a `.mofo` file:
   ```mofo
   name: str = "John"
   printf(name)
   ```
2. Run the file:
   ```bash
   mofo your_file.mofo
   ```
   Output:
   ```plaintext
   John
   Execution completed.
   ```

### CLI Options
- `--version`: Displays the version of the Mofo language.
- `--help`: Provides usage instructions.

### Syntax
- **Variable Declaration**:
  ```mofo
  variable_name: type = value
  ```
  Example:
  ```mofo
  greeting: str = "Hello, World!"
  ```
- **Printing**:
  ```mofo
  printf(expression)
  ```
  Example:
  ```mofo
  printf(greeting)
  ```

## Contributing
We welcome contributions! To get started:
1. Fork the repository and clone it locally.
2. Make your changes and add tests in the `tests/` directory.
3. Run the tests:
   ```bash
   cargo test
   ```
4. Submit a pull request with a clear description of your changes.

## License
This project is licensed under the [MIT License](LICENSE).

## Contact
For any questions or feedback, feel free to open an issue or contact the maintainer.

---

---

### Key Sections Explained
1. **Features**: Highlights what the language can do.
2. **Installation**: Explains how to install using the installer or from source.
3. **Usage**: Demonstrates how to use the language with examples.
4. **Contributing**: Encourages open-source contributions.
5. **License**: Mentions the license for the package.

You can further customize this based on your project's goals. Let me know if you'd like to adjust any part!