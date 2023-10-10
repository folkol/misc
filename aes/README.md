# AES

Implementation of AES (Inv)Cipher + Helpers Functions.

- only 128bit keys
- test cases come from AES Spec Appendix
- pre-calculated lookup tables from Wikipedia

## References

- https://www.hrpub.org/download/20171130/CSIT2-13510193.pdf
- https://en.wikipedia.org/wiki/Rijndael_MixColumns#Galois_Multiplication_lookup_tables
- https://csrc.nist.gov/files/pubs/fips/197/final/docs/fips-197.pdf.
- https://crypto.stackexchange.com/questions/10996/how-are-the-aes-s-boxes-calculated
- https://books.google.se/books/about/The_Design_of_Rijndael.html?id=tfjd6icCUoYC&redir_esc=y

## Random notes

- S-box is (GF) inverse + affine transform
- (Inv)Cipher uses a round function that is composed of
  - byte substitution (S-box)
  - shifting rows of the state
  - mixing the data within each column of the state
  - adding (XOR) a round key to the state
- 

## Glossary (from spec)

| Term                  | Description                                                                                                                                                                                |
|-----------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| AES                   | Advanced Encryption Standard                                                                                                                                                               |
| Affine Transformation | A transformation consisting of multiplication by a matrix followed by the addition of a vector.                                                                                            |
| Block                 | Sequence of binary bits that comprise the input, output, State, and Round Key. The length of a sequence is the number of bits it contains. Blocks are also interpreted as arrays of bytes. | 
| Cipher                | Series of transformations that converts plaintext to ciphertext using the Cipher Key.                                                                                                      |
| Cipher Key            | Secret, cryptographic key that is used by the Key Expansion routine to generate a set of Round Keys; can be pictured as a rectangular array of bytes, having four rows and Nk columns.     |
| Ciphertext            | Data output from the Cipher or input to the Inverse Cipher.                                                                                                                                |
| Inverse Cipher        | Series of transformations that converts ciphertext to plaintext using the Cipher Key.                                                                                                      |
| Key Expansion         | Routine used to generate a series of Round Keys from the Cipher Key.                                                                                                                       |
| Plaintext             | Data input to the Cipher or output from the Inverse Cipher.                                                                                                                                |
| Rijndael              | Cryptographic algorithm specified in this Advanced Encryption Standard (AES).                                                                                                              |
| Round Key             | Round keys are values derived from the Cipher Key using the Key Expansion routine; they are applied to the State in the Cipher and Inverse Cipher.                                         |
| State                 | Intermediate Cipher result that can be pictured as a rectangular array of bytes, having four rows and Nb columns.                                                                          |
| S-box                 | Non-linear substitution table used in several byte substitution transformations and in the Key Expansion routine to perform a one- for-one substitution of a byte value.                   |
| Word                  | A group of 32 bits that is treated either as a single entity or as an array of 4 bytes.                                                                                                    |
