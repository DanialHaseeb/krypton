# Krypton

Krypton is a command-line tool that provides various cryptographic operations. It supports different encryption schemes, including the Caesar cipher, Affine cipher, and Enigma machine.

---

### Table of Contents

- [Features](#features)
- [Usage](#usage)
   - [Cloning the Repository](#cloning-the-repository)
   - [Building the Project](#building-the-project)
   - [Running Krypton](#running-krypton)
- [Cryptanalysis](#cryptanalysis)
   - [Brute Force](#brute-force)
   - [Index of Coincidence](#index-of-coincidence)
- [Implementation Details](#implementation-details)
   - [phf](#phf)
   - [Program Flow](#program-flow)
- [Examples](#examples)
   - [Encrypt using Caesar cipher](#example-encrypt-using-caesar-cipher)
   - [Decrypt using Affine cipher](#example-decrypt-using-affine-cipher)
   - [Cryptanalysis of Enigma](#example-cryptanalysis-of-enigma)
- [References](#references)

---

## Features

- Encryption and decryption using different encryption schemes.
- Cryptanalysis to break encryption and recover plaintext.
- Command-line interface for easy interaction.

## Usage

1. Clone the repository:

   ```shell
   git clone https://github.com/DanialHaseeb/krypton.git
   ```

2. Build the project using the Rust package manager, Cargo:

   ```shell
   cd krypton
   cargo build --release
   ```

3. Run the desired cryptosystem with the specified mode:

   ```shell
   ./krypton [mode] [encryption-scheme] [key] < [input-file]
   ```

   Replace `[mode]` with the desired mode of operation. The available modes are:

   - `encrypt`: Performs encryption using the specified encryption scheme.
   - `decrypt`: Performs decryption using the specified encryption scheme.
   - `analyse`: Attempts to cryptanalyze the provided ciphertext and recover the plaintext.

   Replace `[encryption-scheme]` with the desired encryption scheme. The available encryption schemes are:

   - `caesar`: Caesar cipher, a simple substitution cipher where each letter in the plaintext is shifted by a fixed number of positions.
   - `affine`: Affine cipher, a type of substitution cipher that combines the Caesar cipher with modular arithmetic.
   - `enigma`: Enigma machine, a complex encryption device used during World War II.

   Replace `[key]` with the necessary key for the chosen encryption scheme. The key requirements depend on the selected encryption scheme. See the specific instructions for each encryption scheme below.

   Instead of passing the input directly on the command line, you can use input redirection via the `<` symbol to read the input from a file. Replace `[input-file]` with the path to the file containing the input data.

### Caesar Cipher Key Format

When using the Caesar cipher, the key should be an integer representing the shift value. For example, a key of 3 indicates a shift of 3 positions. Note that only 26 unique shift values exist (0-25), however we've engineered our program in a way that it accepts any number and maps it to one of these values using the mod 26 function.

### Affine Cipher Key Format

When using the Affine cipher, the key should consist of two integers separated by a space. The first integer represents the factor, and the second integer represents the shift value. Similar to Caesar cipher, the factor and shift values too can have only 26 unique values (0-25) each too and use the mod 26 function again to map inputs to them.

### Enigma Machine Key Format

When using the Enigma machine, the key should be provided in a specific format representing the configuration of the machine. The key consists of the following components:

- **Rotors**: The configuration of the three rotors used in the machine. Each rotor is specified by its kind (e.g., `I`, `II`, `III`), initial position, and ring setting. For example, `I A A` represents rotor kind I with the initial position set to 0 and ring setting set to 0.

- **Reflector**: The type of reflector used in the machine. The reflector type should be specified. For example, `A` represents reflector type A.

- **Plugboard**: The connections made in the plugboard. The plugboard connections should be specified as pairs of letters separated by a space. For example, `AB CD EF` represents connections between letters `A` and `B`, `C` and `D`, and `E` and `F`.

Here's an example of how to run the Enigma machine with a specific key configuration:

```shell
./krypton encrypt enigma I A B II C D III E F A AB CD -- < input.txt
```

*Note: Each encryption scheme may have specific key requirements and usage instructions. Make sure to provide the correct key format according to the selected encryption scheme.*

## Cryptanalysis

### Brute Force

The Caesar and Affine ciphers can be broken using a brute-force method by exhaustively searching the entire key space. Since the key space for these ciphers is relatively small, it is feasible to try all possible keys and determine the one that produces the best decryption.

Here's how the brute-force method works:

1. Generate a list of all possible keys for the cipher. For the Caesar cipher, the key space consists of all possible shift values (0-25). For the Affine cipher, the key space consists of all possible combinations of factor and shift values.

2. For each key in the key space, perform decryption on the ciphertext using the selected cipher and key.

3. Score the resulting decryption by counting the number of valid English words it contains. To do this, perform a binary search ([dictionary::contains](https://github.com/DanialHaseeb/krypton/blob/main/src/caesar/analyse/dictionary.rs)) on an English dictionary. If a word is found, increment the score.

4. Keep track of the key and score for each decryption.

5. After trying all keys, select the key with the highest score as the most likely key.

6. Use the selected key to decrypt the ciphertext and recover the plaintext.

By exhaustively searching the entire key space and scoring each decryption, the brute-force method allows us to find the key that produces the most plausible decryption in terms of valid English words. This approach takes advantage of the fact that the English language has certain characteristic word frequencies and patterns that can be used as indicators for correct decryption.

Although the brute-force method guarantees finding the correct key, it can be computationally expensive, especially for longer ciphertexts. However, for shorter texts or cases where the key space is small, it remains a practical and effective technique for breaking the Caesar and Affine ciphers.

## Cryptanalysis

### Index of Coincidence

In the `analyse` mode, Krypton utilizes the _Index of Coincidence_ method to perform cryptanalysis and recover the plaintext. The Index of Coincidence measures the probability that two randomly selected letters from the ciphertext are the same. By analyzing the frequency distribution of letters in the ciphertext, we can exploit the inherent characteristics of the English language to break encryption.

Here's a step-by-step guide on how the Enigma machine can be broken using the Index of Coincidence method:

1. Gather a sufficiently large amount of encrypted ciphertext. The more ciphertext available, the better the chances of breaking the encryption.

2. Determine the probable settings of the Enigma machine used for encryption. This includes the rotor types, initial positions, ring settings, and plugboard connections.

3. Generate a frequency distribution of letters in the ciphertext. Count the occurrences of each letter to identify the relative frequencies.

4. Calculate the Index of Coincidence (IoC) for the ciphertext. The IoC is computed using the formula:

   ```
   IoC = Σ (ni * (ni - 1)) / (N * (N - 1))
   ```

   Where `ni` represents the frequency of the `i`-th letter, and `N` is the total number of letters in the ciphertext.

   The IoC ranges from 0 to 1, where higher values indicate a higher probability of two randomly selected letters being the same. In English, the IoC is typically around 0.067.

5. Compare the calculated IoC with the expected IoC for the English language. If the calculated IoC is close to the expected value, it suggests that the probable settings of the Enigma machine have been found. Otherwise, continue with the next steps.

6. Modify the probable settings of the Enigma machine, such as rotor types, initial positions, ring settings, and plugboard connections. Repeat steps 3 to 5 for the new settings.

7. Iterate through various settings and compute the IoC for each configuration. Keep track of the settings that yield an IoC closest to the expected value.

8. Once the probable settings of the Enigma machine have been determined, apply those settings to the Enigma machine simulation.

9. Decrypt the ciphertext using the simulated Enigma machine with the probable settings. The resulting plaintext is the recovered message.

Breaking the Enigma machine involves an iterative process of adjusting the settings and computing the IoC. By narrowing down the settings that yield an IoC close to the expected value, the Enigma machine can be effectively cryptanalyzed.

It's important to note that the success of breaking the Enigma machine heavily relies on having a sufficient amount of ciphertext and knowledge of the encryption settings. Additionally, brute-force methods, such as trying all possible combinations of settings, can be computationally expensive for larger key spaces. However, the Index of Coincidence method provides a powerful technique to analyze the frequency patterns in the ciphertext and aid in the decryption of the Enigma machine.

### Other Implementation Details

#### `phf`

We leverage the power of the [`phf` crate](https://crates.io/crates/phf), a compile-time hash table generator for Rust, to facilitate efficient translation from letters to numbers. By creating a hash map at compile time, we can eliminate runtime overhead and ensure fast lookups during the cryptographic operations. The `phf` crate allows us to define a perfect hash function, enabling us to generate a minimal perfect hash table that maps each letter to its corresponding numerical representation. This approach not only improves the performance of our encryption and decryption algorithms but also enhances the overall coding experience by providing seamless and quick translations between letters and numbers.

#### Programme Flow

The Krypton application follows a structured program flow to handle different encryption schemes and modes of operation. Here's an overview of how it works:

1. **krypton.rs**: The main entry point of the application, it utilizes the `std::env` and `std::process` crates to read command-line arguments and interact with the operating system. It parses the command-line arguments and passes them to the `Config` struct for further processing.

2. **config.rs**: The `Config` struct represents the configuration of the encryption operation. It consists of `Mode` and `Scheme` structs. The `parse` function in `Config` takes the command-line arguments and delegates the parsing to the corresponding `parse` functions in the `Mode` and `Scheme` structs. It returns a `Result` object containing the parsed configuration.

3. **mode.rs**: The `Mode` enum represents the mode of operation for the encryption. It can have three values: `Encrypt`, `Decrypt`, or `Analyse`. The `parse` function in `Mode` takes the command-line arguments and determines whether the application should perform encryption, decryption, or cryptanalysis based on the provided mode. If the mode is not recognized or no mode is provided, it displays an error message indicating an unknown or missing mode of operation.

4. **scheme.rs**: The `Scheme` enum represents the encryption scheme to be used, including `Caesar`, `Affine`, and `Enigma`. The `parse` function in `Scheme` takes the command-line arguments and determines the encryption scheme based on the provided input. If the scheme is not recognized or no scheme is provided, it displays an error message indicating an unknown or missing encryption scheme.

5. **caesar / affine / enigma.rs**: This module handles the encryption and decryption operations for the respective cipher. It uses the `std::error::Error` trait for error handling. If the mode is set to `Analyse`, it invokes the `run` function of the `Analyse` struct to perform cryptanalysis. Otherwise, it extracts the key from the command-line arguments and creates a `Key` struct, representing the shift value for the cipher. Depending on the mode, it calls the `run` function of either the `Encrypt` or `Decrypt` struct with the key argument.

The program flow described above provides a high-level understanding of how the Krypton application processes command-line arguments, determines the mode and encryption scheme, and executes the appropriate encryption, decryption, or cryptanalysis operations. It ensures that the application functions correctly based on the user's input, handling errors and providing helpful messages when needed.

## Examples

### Example: Encrypt using Caesar cipher

```shell
./krypton encrypt caesar 3 < plaintext.txt
```

This command encrypts the content of the `plaintext.txt` file using the Caesar cipher with a shift of 3 positions and outputs the ciphertext.

### Example: Decrypt using Affine cipher

```shell
./krypton decrypt affine 5 7 < ciphertext.txt
```

This command decrypts the content of the `ciphertext.txt` file using the Affine cipher with a factor of 5 and a shift of 7 positions and outputs the plaintext.

### Example: Cryptanalysis of Enigma

```shell
./krypton analyse enigma I A B II C D III E F A AB CD -- < ciphertext.txt
```

This command attempts to cryptanalyze the content of the `ciphertext.txt` file encrypted with the Enigma machine using the provided key configuration. It uses the Index of Coincidence method to recover the plaintext.


## References

This project was inspired by the fascinating history of cryptography, particularly the Enigma machine used during World War II, as depicted in the movie _The Imitation Game._ The following resources were extensively used throughout this project:

- [Introduction to Enigma](https://youtu.be/G2_Q9FoD-oQ)
- [Flaw in Enigma](https://youtu.be/V4V2bpZlqx8)
- [Cracking Enigma in 2021](https://youtu.be/RzWB5jL5RX0)
- [Mike Pound's Enigma machine implementation](https://github.com/mikepound/enigma)
- [Cryptanalysis of Enigma](https://web.archive.org/web/20060720040135/http://members.fortunecity.com/jpeschel/gillog1.htm)
- [Affine Cipher - Interactive Maths](https://crypto.interactive-maths.com/affine-cipher.html)
- [Affine Cipher - Arizona State University](https://math.asu.edu/sites/default/files/affine.pdf)
- [Caesar cipher - Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher)

These resources provided valuable insights into the various encryption schemes and techniques used in this project.
