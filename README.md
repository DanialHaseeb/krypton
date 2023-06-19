# krypton
A Rust implementation of Enigma (and other classical cryptosystems) as well as their cryptanalyses.

<Still drafting; don't have a heart attack reading this...>
<Update: STILL drafting... this is going to take a while...>
<Update: STILL drafting... I give up...>

📦 Crates & Libraries
std::env
use std::io
std::process
std::error::Error
phf
siphasher


Tell about input.txt and output.txt files


Overview:
This project focuses on Caesar, Affine and Enigma cryptography techniques. For each of these, it provides the options of encrypting a plain text into cipher text using one of these techniques and a 'key' value, decrypting cipher text into plain text given a similar 'key', or analyzing a given cipher text and finding a key to extract the plain text a.k.a 'break' the cipher. The project also includes implementations of different techniques of 'scoring' valid words obtained from deciphering the text including binary search, hashing and index-of-coincidence approaches.



Used:
Enums, Box, Option, HashMap, Map, Traits, Iterators

Libraries
Algorithm
Techniques
Break, Encrypt, Decrypt explanation each
Referneces --- how Enigma got brokenn
Input format a.k.a Commands(???)


Program flow (I guess):

Krypton.rs:
Uses std::env & std:: process crates.
Read command line arguments and parse into config struct.

Config.rs:
Config is a struct composed of Mode and Scheme structs.
Has parse function which takes command line arguments and sends them to "Mode" and "Scheme" struc t parse functions.
Returns result object of Config struct composed of result of mode and Scheme parse functions.

Mode.rs:
Mode is an enum with values Encrypt, Decrypt, Analyse
has parse function takes command line arguments and determines if we want to encrypt, decrypt or break the cipher.
If anything other than these values or if no mode input given, we get error messages "Unknown mode of operation. 🤔" or "No mode of operation provided. 🧐"

Scheme.rs:
Scheme is an enum with values Caesar, Affine, Enigma
has parse function which takes command line arguments and determines if the encryption/decryption/breaking algorithm we are applying is for the Affine, Caeser or Enigma scheme
If anything other than these values or if no mode input given, we get error messages "scheme => Unknown scheme. 🤔" or "scheme => No encryption scheme provided. 🧐"

Caesar.rs:
uses std::error::Error
Takes Mode and command line input arguments
If the mode is analyse/break, then we use the run function of analyse struct
else, we extract the key from the command line arguments into a key struct.
This is a 'shift' value in the case of Caesar.
We call encrypt struct's run function with this key argument, or decrypt struct's run function with this key argument depending on whether the mode is encrypt of decrypt.


Analyse.rs:
Has dictionary.rs---> has `contains` functions which finds given word in dictionary using binary search ---> Make binary search sound big big biggg

# Krypton

Krypton is a command-line tool that provides various cryptographic operations. It supports different encryption schemes, including the Caesar cipher, Affine cipher, and Enigma machine.

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

When using the Caesar cipher, the key should be an integer representing the shift value. For example, a key of 3 indicates a shift of 3 positions.

### Affine Cipher Key Format

When using the Affine cipher, the key should consist of two integers separated by a space. The first integer represents the factor, and the second integer represents the shift value.

### Enigma Machine Key Format

When using the Enigma machine, the key should be provided in a specific format representing the configuration of the machine. The key consists of the following components:

- **Rotors**: The configuration of the three rotors used in the machine. Each rotor is specified by its kind (e.g., `I`, `II`, `III`), initial position, and ring setting. For example, `I A A` represents rotor kind I with the initial position set to 0 and ring setting set to 0.

- **Reflector**: The type of reflector used in the machine. The reflector type should be specified. For example, `A` represents reflector type A.

- **Plugboard**: The connections made in the plugboard. The plugboard connections should be specified as pairs of letters separated by a space. For example, `AB CD EF` represents connections between letters `A` and `B`, `C` and `D`, and `E` and `F`.

Here's an example of how to run the Enigma machine with a specific key configuration:

```shell
./krypton -- encrypt enigma I A B II C D III E F A AB CD -- < input.txt
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

#### Other Implementation Details

##### `phf`

In Krypton, we leverage the power of the `phf` crate, a compile-time hash table generator for Rust, to facilitate efficient translation from letters to numbers. By creating a hash map at compile time, we can eliminate runtime overhead and ensure fast lookups during the cryptographic operations. The `phf` crate allows us to define a perfect hash function, enabling us to generate a minimal perfect hash table that maps each letter to its corresponding numerical representation. This approach not only improves the performance of our encryption and decryption algorithms but also enhances the overall user experience by providing seamless and quick translations between letters and numbers.
