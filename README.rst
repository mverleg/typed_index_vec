
Typed-index vector
===============================

I wanted a vector type that takes a typed index that only works for that vector.

E.g. a vector of `Person`s can only be indexed using `Id<Person>`.

This helps prevent mistakes:

* One cannot accidentally look up a `House`'s id in a vector of `Person`s.
* One cannot e.g. take the sum of two ids, which would be meaningless.

**This is not a product-ready solution.**


