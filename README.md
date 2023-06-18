# krypton
A Rust implementation of Enigma (and other classical cryptosystems) as well as their cryptanalyses.

<Still drafting; don't have a heart attack reading this...>
<Update: STILL drafting... this is going to take a while...>

📦 Crates & Libraries
phf
siphasher
In krypton.rs: std::env, std::process


Tell about input.txt and output.txt files




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

References:
https://youtu.be/G2_Q9FoD-oQ
https://youtu.be/V4V2bpZlqx8
https://youtu.be/RzWB5jL5RX0
https://github.com/mikepound/enigma
https://web.archive.org/web/20060720040135/http://members.fortunecity.com/jpeschel/gillog1.htm
