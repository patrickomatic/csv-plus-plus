# Ruby
- [ ] Make it compatible with older ruby versions - no reason we need the newest (it seems like 2.3 would be reasonable to target)
- [x] Look into using Sorbet

# Testing/Publishing

- [ ] Homebrew package
- [ ] Debian package
- [x] Get the Google API default credentials working with tests

# Spreadsheet Functionality

- [ ] OpenOffice - whatever it's equivalent format is
- [ ] Charts
- [ ] Pivot table support - not sure what this would even look like
- [ ] Apple Numbers - I don't think there are even libraries to do it
- [ ] Support for vb script?  I don't think it would work anywhere but excel
- [x] Excel - look into caxlsx, Apache POI or openxml
- [x] make it output to CSV (loses formatting, but keeps formulas, variables and expansions)

# Modifiers

- [ ] shorthand conventions - for example (format=bold == f=b, bg=#000000, fs=20 (fontsize=20)
- [ ] conditional formatting
- [ ] implement freeze
- [ ] data validation
- [x] make var= work better with expand=.  if it's on a row that then it should just bind relative to the current row... (use celladjacent?)
- [x] bind - assign this cell to a variable (you can't do this in an expand)
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

- [ ] Make the csv-separator work with '|' instead. I think it would look better
- [ ] Make the merge strategy a configurable flag (whether it overwrites data already in the spreadsheet or not)
  * maybe even make it warn when in --verbose mode
- [ ] if you reference a variable outside of an expand, it references the range of the expand
- [ ] a way to not show a function if it's dependent cells are blank (make it less ugly when expanded out)
- [x] can I get rid of $$ syntax and just resolve the functions and runtime vars it knows about?
- [x] make the code section (and `---`) optional
- [x] maybe a RuntimeValue doesn't belong as an entity - it doesn't output itself, it only needs to exist during function resolution
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

- [ ] Allow it to take URLs to files (like `-o https://github.com/patrickomatic/foo/some_file.csvpp`)
- [x] Revise the "how to use" docs
- [x] backups before each write
- [x] Benchmarks in verbose mode
- [x] Simplecov
- [X] Rubocop
- [x] create the spreadsheet if it doesn't exist
- [x] verbose mode
