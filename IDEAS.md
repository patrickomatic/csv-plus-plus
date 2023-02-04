# Testing
- [ ] Get the Google API default credentials working

# Language
- [x] Get infix operators working

# Formats
- [ ] Excel - look into caxlsx, Apache POI or openxml
- [ ] OpenOffice - whatever it's equivalent format is
- [ ] Apple Numbers
- [x] make it output to CSV (loses formatting, but keeps formulas, variables and expansions)

# Modifiers to rows & cells

- [ ] conditional formatting
- [ ] data validation
- [ ] freeze
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

# Variables and functions

- [ ] a way to not show a function if it's dependent cells are blank (make it less ugly when expanded out)
- [ ] SHEETREF()
- [x] CELLREF()
- [x] user defined functions at the top
- [x] `$$row` current row

# Performance

- [ ] concurrently process each cell?
- [x] memoization and lazy evaluation as much as possible

# Other

- [ ] Revise the "how to use" docs
- [ ] Charts
- [ ] Pivot table support - not sure what this would even look like
- [ ] backups before each write
- [x] Benchmarks in verbose mode
- [x] Simplecov
- [X] Rubocop
- [x] create the spreadsheet if it doesn't exist
- [x] verbose mode
