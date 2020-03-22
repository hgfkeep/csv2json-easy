# csv to json

👏Welcome to use and suggest!

🙋‍♂️This is a cli tools for **converting csv to json**.


## Install

```bash
cargo install csv2json-easy
```

## Usage

help information(with `csv2json --help`): 


```bash
convert csv to json format 0.0.1
heguangfu
csv to json tools

USAGE:
    csv2json [FLAGS] [OPTIONS] <stdin_csv>

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     output json pretty format
    -V, --version    Prints version information
    -v, --verbose    get some detail infomation od csv to json

OPTIONS:
    -i, --input-file <input_csv>       input csv file path, either input_csv or stdin_csv is required!
    -l, --limit <limit>                output ${limit}json objects with 
    -s, --offset <offset>              output json objects start from ${offset} [default: 0]
    -o, --output-file <output_json>    result json file path

ARGS:
    <stdin_csv>    stdin csv content, either input_csv or stdin_csv is required!
```

⚠️Note:

* `input-file` or `stdin_csv` is required when processing csv.
* default output to stdout
* you can use `limit` and `offset` to proccess part of csv.


## Example

### convert a csv file to data.json

```
cat data/data.csv | csv2json-easy > data.json
```

or

```
csv2json-easy -i data/data.csv -o data.json
```

> Note: CAREFUL with `-v` option when using linux tunnel redirect stdout to a file .

### want to process part of csv


```
cat data/data.csv | csv2json-easy -l 10 -s 2> data.json
```

or

```
csv2json-easy -i data/data.csv -l 10 -s 2 -o data.json
```

### verbose

use `-v` will output some verbose information.