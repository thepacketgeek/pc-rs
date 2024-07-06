# `pc` (print column) CLI utility

A simple utility to print the desired column from tabular data, to replace verbose `awk` invocations:

`ls -l | awk '{ print $2 }'` becomes `ls -l | pc 2`

## Inputs
`pc` can read from stdin:

```
$ ls -l | pc 2

# OR

$ pc 1 < ls -l
```

as well as a given filepath:

```
$ pc 1 ~/data.txt
```

## Delimiter
You can specify the delimiter `pc` uses to split lines into columns, with the default being space. Any consecutive delimiter characters will be considered a single column delimiter:

`"test1 test2"` will be split the same as `"test1    test2"`

You can provide alternate delimiters with the `--delimiter` option:

```
$ echo "1,2,3,4" | pc 2 --delimiter ,
2
```

## Output Separator
The default output separator is a newline:

```
$ echo "1 2 3 4\na b c d" | pc 2
2
b
```

but you can provide a different character to print between each matching column:

```
$ echo "1 2 3 4\na b c d" | pc 2 --separator "|"
2|b|%
```

 # License

`pc-rs` is both MIT and Apache License, Version 2.0 licensed, as found
in the LICENSE-MIT and LICENSE-APACHE files.