shw
===

`cat + ls ==> shw`

Acts like `ls` when run without argument or if argument provided is path to directory. Acts like `cat` when argument provided is file.

```sh
Usage: shw [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to directory or file [default: .]

Options:
  -l             Show directory entry statistics
  -a             Show hidden files (dot files)
  -h, --help     Print help
  -V, --version  Print version
```