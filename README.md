# Review of Cryptopal Chapter 1 challenges 1-6 (Vigenere)
2025-12-22

#### Goals
I challenged myself to tackle Cryptopals in Autumn/Winter 2025 for these reasons:
- I wanted to scrape the mould of my Rust skills
- I wanted to get my hands dirty with real bytes rather than lofty abstractions
- I wanted to learn Neovim
- I wanted something low-stakes I could chip away at before or after work

I did challenges 1-6. Still need to do 7 & 8 to finish the first chapter but decided to pause after Vigenere.
The reason I have decided to pause is because I have achieved my goals, and I now want to spend my time doing other things.

#### Things I learned about cryptography:
- XOR has this awesome characteristic in that it is reversible.  If A XOR B = C, then A XOR C = B and B XOR C = A.
- Cryptography is all about the fiddly bits. paddings, line breaks, character sets.
- The Vigenere code breaker is very elegant.

#### Things I learned about Rust:
- Loads of language features
- How to structure a crate
- Idiomatic coding
- Clap command line crate
- thiserror crate
- string chars vs bytes

#### Summary
Overall I am proud of myself for finding the time to do these challenges. I learned a lot of Rust skills and neovim skills. I picked up some beginner level cryptography too which is nice. The challenges are very nicely framed. I would recommend others to try out [cryptopals](https://cryptopals.com/).
