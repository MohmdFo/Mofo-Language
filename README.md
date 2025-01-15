# Mofo Language

Mofo Language is a static-typed, interpreted programming language inspired by Python. It offers Python-like syntax with built-in support for `printf` and static typing. Variables must be declared with their type during assignment.

## Features
- **Static Typing**: Variables are declared with a type (e.g., `name: str = "value"`).
- **Python-like Syntax**: Easy to read and write.
- **Built-in `printf` Function**: Print output to the terminal.
- **Variable Declaration and Usage**: Supports types like `str`.

## Installation

### From Source (Recommended)
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/mofo-language.git
   cd mofo-language
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Install the language:
   ```bash
   cargo install --path .
   ```
4. Verify installation:
   ```bash
   mofo --version
   ```
   Expected output:
   ```plaintext
   Mofo Language v0.1.0
   ```

### From Installer (Alternative)
1. Download the latest `setup.exe` from the [Releases](#) section.
2. Run the installer and follow the instructions.
3. **Update PATH Manually**:
   - **If using Git Bash or similar**:
     1. Open the `.bashrc` or `.bash_profile` file in your home directory:
        ```bash
        nano ~/.bashrc
        ```
     2. Add the following line:
        ```bash
        export PATH=$PATH:/c/Program\ Files\ \(x86\)/MofoLanguage
        ```
     3. Save the file and reload it:
        ```bash
        source ~/.bashrc
        ```
     4. Test:
        ```bash
        mofo --version
        ```

   - **If using Windows Command Prompt or PowerShell**:
     1. Open **Environment Variables**:
        - Right-click on "This PC" or "My Computer" and select **Properties**.
        - Click on **Advanced system settings**.
        - In the **System Properties** window, go to the **Advanced** tab and click on **Environment Variables**.
     2. Edit the `PATH` variable:
        - In the **System Variables** section, find `Path` and click **Edit**.
        - Add the Mofo Language installation directory, e.g.:
          ```
          C:\Program Files (x86)\MofoLanguage
          ```
        - Click **OK** to save the changes.
     3. Restart your terminal or command prompt for changes to take effect.
     4. Test:
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
