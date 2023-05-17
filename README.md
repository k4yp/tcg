![TCG](tcg.svg)

# Template Code Generator

![](https://img.shields.io/crates/d/tcg)
![](https://img.shields.io/crates/v/tcg)
![](https://img.shields.io/crates/l/tcg)

#### Generate template code for competitive programming.

Lightweight Rust based command line utility to automate the creation of problem files for competitive programming or practice. It generates problem files based on customs templates.

## Prerequisites

-   Cargo package manager should be installed on your system.
-   Make sure to have `~/.cargo/bin/` as a PATH variable.

## Usage

The program expects the following command-line arguments:

```
tcg <name> -t <template>
```

Optionally provide an input output file

```
tcg <name> -t <template> -i <input> -o <output>
```

-   `<name>`: The name of the problem. This will be used to create a directory with the same name to store the problem files.
-   `-l, --language`: The language extension of your problem solution.
-   `-i, --input`: The input file of the problem solution. You can use `%` as a placeholder to be replaced with the problem name.
-   `-o, --output`: The output file of your problem solution. You can use `%` as a placeholder to be replaced with the problem name.
-   `-h, --help`: Print the help screen, which displays the program's usage and available options.

## Example

To generate problem files for a problem named "example" with the language extension "py", an input file named "example.in", and an output file named "example.out", you would run the following command:

```
tcg example -t io.py -i %.in -o %.out
```

This command will create a directory named "example" and generate the necessary problem files inside it.

## Templates

List of the default template files. All of these are customizable. Use `%input%` and `%output%` as placeholders for input and output files.

**Python**  
io.py

```python
with open("%input%","r") as f:
    case = f.read()

with open("%output%","w") as f:
     f.write(case)
```

cli.py

```python
case = input()

print(case)
```

## License

This program is licensed under the [MIT License](https://opensource.org/licenses/MIT).
