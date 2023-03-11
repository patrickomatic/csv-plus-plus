# Testing/Publishing

- [ ] Homebrew package
- [ ] Debian package
- [x] Get the Google API default credentials working with tests

# Spreadsheet Compatibility

- [ ] OpenOffice - whatever it's equivalent format is
- [ ] Apple Numbers - I don't think there are even libraries to do it
- [x] Excel - look into caxlsx, Apache POI or openxml
- [x] make it output to CSV (loses formatting, but keeps formulas, variables and expansions)

# Modifiers 

- [ ] bind - assign this cell to a variable (you can't do this in an expand)
- [ ] conditional formatting
- [ ] data validation
- [ ] freeze
- [x] make horizontal and vertical alignments different things. right now it's ambiguous that they share 'center'.  also maybe add 'justified' and 'distributed'
- [x] make cell-level modifiers override row-level
- [x] data validation
- [x] fonts
- [x] vertical alignment
- [x] hyperlinks
- [x] note
- [x] border options
- [x] ranges that rows can apply expand to
- [x] horizontal alignment
- [x] formatting: bold, italic, underline

# Language

- [ ] can I get rid of $$ syntax and just resolve the functions and runtime vars it knows about?
- [ ] a way to not show a function if it's dependent cells are blank (make it less ugly when expanded out)
- [ ] turn it into a lisp???
- [x] ROWABOVE - given 2 (or 2A, $2A, etc) returns 1
- [x] ROWBELOW - given 2 (or 2A, $2A, etc) returns 3
- [x] CELLABOVE - given 2B returns 1B
- [x] CELLBELOW - given 2B returns 3B
- [x] Get infix operators working
- [x] CELLREF()
- [x] user defined functions at the top
- [x] `$$row` current row

# Performance

- [ ] concurrently process each cell?
- [x] memoization and lazy evaluation as much as possible

# Other

- [ ] Make it compatible with older ruby versions - no reason we need the newest (it seems like 2.3 would be reasonable to target)
- [ ] Revise the "how to use" docs
- [ ] Charts
- [ ] Pivot table support - not sure what this would even look like
- [x] backups before each write
- [x] Benchmarks in verbose mode
- [x] Simplecov
- [X] Rubocop
- [x] create the spreadsheet if it doesn't exist
- [x] verbose mode
