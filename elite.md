Rust room

# what is rust?
* speed
* saftey
* cargo (pip but for rust)
* Links to the rust book (the documentation for rust is literally an ebook)

# Toolings


# Introduction
* if statements
* functions
* variables
* loops

Some basic questions like "how would u make variable x, how would u make a function named x" and so on

# Expressions
* rust return statements (they're super weird)
* What is an expression?

# Results object
If something can fail in Rust, you **HAVE** to deal with the exception.

FOr example

```python3
# python
f = open("file.txt", 'r')
```

But in Rust, you **cannot** open a file unless you write an exception for if that file does not exist.

In Rust, everything that can fail returns a <return> object. This means that you're never going to encounter an unexpected "file not found" or similar, because everything is accounted for.

# Borrow checker
Variables not in scope are killed. In Python, I think you can do:

```
if y = 6:
    x = 3
else:
    x = 9
```

But in Rust, the borrow checker will kill X the second the if is over. You cannot create variables inside a scope and use them out fo the scope.

You can however, do this

```
let x = if y == 6 {3} else {9}
```

This is a security feature, the borrow checker stops you from doing silly things with variables -- everything is either in scope or it's killed off.

# Multi-threading
* Rayon
* Turning a for loop into multi-threaded with one word (into_iter -> into_para_iter)
# Async
* Basic async 101
* Making a web server with async rust and how rust ensures saftey with async


# Packaging
* Packaging your rust app
* Okay not security related but it's really cool it's just 1 command `rust publish`

## TODO
* Rust has an official discord (like, the one the Rust team uses to organise their codebase and create new features). I can ask them for their fave blueteam features of Rust :)
* Ask if I'm allowed to include a link to the official discord of Rust. They only have like 2 help channels, the rest of the channels are for organising new features / work on the language itself. It's really cool to be able to see the language evole in Discord, rather than whatever C++ or Python uses (mailing lists I think)