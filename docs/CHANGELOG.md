## v0.7.0

### Bugfixes

* Fix a bug causing double-quoted strings (i.e, `""foo""`)

### **Breaking Changes**

* Double-quoted strings now use double-quote (`""`) to escape a single-quote.  Previously it used 
  `\"` to escape.
