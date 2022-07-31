# `runjs`

A repository for the [Roll your own JavaScript runtime](https://deno.com/blog/roll-your-own-javascript-runtime) blog post.

**Updated to demonstrate what it looks like to use the [`deno_runtime`](https://github.com/denoland/deno/tree/main/runtime) crate, which includes Deno's ops, instead of the low-level `deno_core` crate.**

```console
❯ cargo run -q
Hello Deno!
Boom!
Unable to read file ./log.txt Error: No such file or directory (os error 2)
    at async Object.readTextFile (deno:deno_runtime-0.71.0/js/40_read_file.js:55:20)
    at async file:///Users/mathias/devel/denoland/roll-your-own-javascript-runtime/example.js:6:20
Read from a file ./log.txt contents: I can write to a file.
Removing file ./log.txt
File removed
```

The result is very close to using `deno run` itself:

```console
❯ deno run --allow-read=log.txt --allow-write=log.txt example.js
Hello Deno!
Boom!
Unable to read file ./log.txt NotFound: No such file or directory (os error 2)
    at async Object.readTextFile (deno:runtime/js/40_read_file.js:55:20)
    at async file:///Users/mathias/devel/denoland/roll-your-own-javascript-runtime/example.js:6:20
Read from a file ./log.txt contents: I can write to a file.
Removing file ./log.txt
File removed
```
