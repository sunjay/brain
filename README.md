# brainfuck

[Brainfuck][1] interpreter written in Rust.

## Usage

```
cargo run -- filename
```
where `filename` is the brainfuck program you want to run.

## Examples

```
cargo run -- examples/hello-world.bf
```

## TODO

- [ ] basic interpreter
- [ ] unbuffered output
- [ ] character buffered input (instead of default line-buffered)
- [ ] compiler instead of interpreter (with optimizations)
    - http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html

[1]: http://www.muppetlabs.com/~breadbox/bf/

