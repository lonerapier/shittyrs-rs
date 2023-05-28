# ShittyRS-rs

Implementation of the [Reed-Solomon erasure codes](https://www.cs.cmu.edu/~guyb/realworld/reedsolomon/reed_solomon_codes.html) algorithm in the most shittiest, wrong way possible. Don't read this, implement it yourself, it's surprisingly easy (not decoding bruh) and that's the beauty of this algorithm.

> **Warning**
>
> This might be the worst rust code you ever read, so proceed forward with caution. Kind reader, if you want to scold me anytime about this shitty code, please open an issue.

This is an attempt to understand one of the most widely used algorithm in the history and learn some Abstract algebra, number theory and rust along the way. It's a toy port of the amazing [python implementation](https://github.com/lrq3000/unireedsolomon). This [blog post](https://research.swtch.com/field) from Russ Cox, and [wikiversity article](https://en.wikiversity.org/wiki/Reed%E2%80%93Solomon_codes_for_coders) are one of the best resources out there to understand this legendary algorithm. I'll try to write my own post sometime in the future.

There are much performat and mature implementations out there, if you need it for production purposes. (I'm honored you arrived here).

- [reed-solomon-erasure](https://github.com/rust-rse/reed-solomon-erasure)
- [klauspost's reedsolomon](https://github.com/klauspost/reedsolomon)
- [blackblaze's JavaReedSolomon](https://github.com/Backblaze/JavaReedSolomon)
- [nicolasT's reedsolomon](https://github.com/NicolasT/reedsolomon)

## TODO

- [x] Implement Field and Polynomial traits.
- [x] create a struct of field::elem and then implement traits on that, eg: addassign, subassign, etc.
- [ ] write tests for traits
- [ ] implement gf_16 backend
- [ ] Implement Berklemassamp decoding algorithm
- [ ] Implement FFT based encoding/decoding
- [ ] Implement [montgomery backend](https://cp-algorithms.com/algebra/montgomery_multiplication.html)

## Acknowledgments

- Awesome [KZG](https://github.com/lambdaclass/lambdaworks_kzg) library by LamdaClass.
- [Unireedsolomon](https://github.com/lrq3000/unireedsolomon): best implementation out there.

