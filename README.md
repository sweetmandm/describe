![generated lines](http://www.davidsweetman.com/images/describe-example.jpg)

## describe

This is an experiment to make line-based generative art.

Right now it just sends the svg to stdout, so you can use it like:

```
$ cargo run > my.svg
```

During experimentation, it can be useful to watch for changes:

```
$ cargo install cargo-watch
$ cargo watch -x 'run > test.svg'
```
