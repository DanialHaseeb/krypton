# krypton
A Rust implementation of Enigma (and other classical cryptosystems) as well as their cryptanalyses.

<Still drafting; don't have a heart attack reading this...>
<Update: STILL drafting... this is going to take a while...>

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
