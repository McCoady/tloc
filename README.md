# tloc (Toad Lines of Code) 🐸
tloc is a lightweight CLI tool built to quickly create a markdown list of all solidity files within a directory, including the SLOC of each file and the total SLOC of the codebase.

Line count should match that of other notable line counters (scc, cloc). 

## Differences vs Other Counters
- Automatically writes the output to a markdown file including all file names sorted by directory with individual line counts.
- Automatically skips directories named "interfaces", "test", "tests", "mock", "mocks"

## Installation
```bash
git clone https://github.com/mccoady/tloc
cd tloc
cargo install --path .
```

## Usage
```bash
tloc
```

**Options**
```bash
# Perform recursive search through any nested directorys within path 
--path # The path to start reading files from, defaults to current location
--out # name of the file to write output to, defaults to scope.md
--no-recurse # whether to not read the contents of nested directories, defaults to false (do read)
```

**Example Output**
```markdown
# Scope
- `FileOne.sol` (203)
- `FileTwo.sol` (603)
TOTAL: 806
```

## Contributing
Contributions are welcome!

