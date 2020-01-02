# Schema Merger

Schema Merger will simply take a file body and apply the changed or updated object/values of another file body.  It is currently only developed for structured schemas and is limited in format, see accepted formats below.

**Features**  
- Merge two schema bodies together
- Accepts JSON and YAML
- Cross format merging for JSON and YAML

_Merge one schema body into another:_

**Input JSON**  
```json
{
  "data": {
    "attributes": {
      "myArray": [
        "a",
        "b",
        "c"
      ]
    }
  }
}
```

**Mixin JSON**  
```yaml
data:
  attributes:
    myArray:
    - d
    - e
    - f
  linked:
    start": 1
    end": 10
```

**Output**  
```json  
{
  "data": {
    "attributes": {
      "myArray": [
        "a",
        "b",
        "c",
        "d",
        "e",
        "f"
      ]
    },
    "linked": {
      "start": 1,
      "end": 10
    }
  }
}
```

## USAGE
### Interface

We provide a command-line utility only with simple usage interface:

```bash
USAGE:
    sm [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -i, --input <input>      Base file path.
    -m, --mixin <mixin>      File path.
    -o, --output <output>    Output to file and leave base file as is.

    Note: Best efforts will be used for format.  Use 'format:path/to'
          notation for strict assignments.

SUBCOMMANDS:
    completions    Completion scripts for various shells.
```

### Accepted Formats

We define schema structures through type enumeration.  Currently we only have handlers for a limited set of schema formats, see the below list for what we support:

- JSON
- YAML

### Mixing schemas

We have tried to make usage flexible for a single purpose utility.

_Only Utility_  
```bash
sm -i input.json -m mixin.json -o update.json
```

_Standard Stream_  

```bash
cat input.json | sm -m mixin.json > update.json
```

### Available shell autocompletion scripts

Auto completion script for the below list of supported shells available:

- Bash
- Zsh
- Fish
- Elvish
- PowerShell

---

## Work In Progress
Feel free to contribute or use in any way.s
