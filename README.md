# Privacy Pass

A Rust implementation of the Privacy Pass protocols as specified in Privacy
Pass IETF WG
[documents](https://datatracker.ietf.org/wg/privacypass/documents/).

The library implements both the server side and the client side components
for the following token types:

 - Privately Verfifiable Tokens
 - Publicly Verfifiable Tokens
 - Batched Tokens

# Limitations

The Privately Verifiable Token type currently uses P256/SHA256 until this
[issue](https://github.com/novifinancial/voprf/issues/81) is resolved.
