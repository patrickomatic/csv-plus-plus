## v0.7.0

### Features

* Build a statically-linked `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` for people
  who are on a system with an outdated glibc

### Bugfixes

* Fix a bug causing double-quoted strings (i.e, `""foo""`)

### **Breaking Changes**

* Double-quoted strings now use double-quote (`""`) to escape a single-quote.  Previously it used 
  `\"` to escape.
