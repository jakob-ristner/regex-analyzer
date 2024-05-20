# Regex Analzyer
This tool was developed for the project in Language-based Security TDA602 at Chalmers Tekniska Högskola. 
Using a semi-complete definition of a regex, it can calculate an upper bound on ambiguity for a given 
regex corresponding NFA, alerting a user of potential exponential ambiguity. 
Such regexes should be avoided, as they may be vulnerable to ReDoS attacks.

## Usage

```console
$ Cargo run "<regex>"
```
