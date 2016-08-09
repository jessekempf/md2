# MD2

An implementation of RSA's MD2 message digest algorithm as described in [RFC 1319](https://www.ietf.org/rfc/rfc1319.txt).

### Why?

1. I wrote a Haskell implementation of MD2 on a lark (it's as slow as it sounds) and decided to see how fast a naïve Rust implementation would be.
2. It's amongst the simplest of the cryptographic message digest algorithms.
3. I wanted to see if MD2 is for reals as slow as it's offhandedly claimed. As it turns out, it is _for reals_ slow. It's about 60 times slower than MD5 for the same file. That's because it processes data byte-by-byte instead of word-by-word.
4. We use cryptographic message digest algorithms almost everywhere. They're often used in preference to CRC32 for data integrity checks, they're the basis of HMAC, and Merkle trees (like what Git's based on top of) need them. I've never had the occasion to implement one.
5. I want to learn Rust when I'm not learning Haskell.

### Limitations
1. It's hella slow. Seriously, it takes the _reference_ implementation, written in 1980s be-as-fast-as-possible-C, a minute to digest a gigabyte of data from an SSD on a 2016-era MacBook Pro. The Rust implementation's about 10% slower because...
2. The MD2 file digesting function is stupid-simple. It reads the entire file into memory before MD2ing it. It trades O(N) memory consumption for a super-simple implementation. Production code would use an O(1) memory consumption technique.
3. It's a toy. Nobody's supposed to use MD2, and this was a learning project to get my hands dirty with Rust.