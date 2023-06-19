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
This project focuses on Caesar, Affine and Enigma cryptography techniques. For each of these, it provides users the option for encrypting a plain text into cipher text using one of these techniques and a 'key' value, decrypting cipher text into plain text given a similar 'key', or analyzing a given cipher text and finding a key to extract the plain text a.k.a 'break' the cipher.



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
Has dictionary.rs---> has contains functions which finds given word in dictionary using binary search ---> Make binary search sound big big biggg

References:
https://youtu.be/G2_Q9FoD-oQ
https://youtu.be/V4V2bpZlqx8
https://youtu.be/RzWB5jL5RX0
https://github.com/mikepound/enigma
https://web.archive.org/web/20060720040135/http://members.fortunecity.com/jpeschel/gillog1.htm

Here's the updated README with the details from `src/enigma/key/rotor.rs`:

## Usage

To use Krypton, follow these steps:

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

- **Rotors**: The configuration of the three rotors used in the machine. Each rotor is specified by its kind (e.g., I, II, III), initial position, and ring setting. For example, `I A A` represents rotor kind I with the initial position set to 0 and ring setting set to 0.

- **Reflector**: The type of reflector used in the machine. The reflector type should be specified. For example, `A` represents reflector type A.

- **Plugboard**: The connections made in the plugboard. The plugboard connections should be specified as pairs of letters separated by a space. For example, `AB CD EF` represents connections between letters A and B, C and D, and E and F.

Here's an example of how to run the Enigma machine with a specific key configuration:

```shell
cargo run --release --bin krypton -- encrypt enigma "I A B II C D III E F A AB CD" < input.txt
```

*Note: Each encryption scheme may have specific key requirements and usage instructions. Make sure to provide the correct key format according to the selected encryption scheme.*
