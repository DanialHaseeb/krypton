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
has parse function.
Determines if we want to encrypt, decrypt or break the cipher.
If anything other than these values or if no mode input given, we get error messages "Unknown mode of operation. 🤔" or "No mode of operation provided. 🧐"



