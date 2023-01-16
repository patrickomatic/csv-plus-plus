# Formats
- [ ] make it output to CSV (loses formatting, but keeps formulas, variables and expansions)
- [ ] Excel - look into Apache POI or openxml
- [ ] OpenOffice - whatever it's equivalent format is

# Modifiers to rows & cells

- [ ] conditional formatting
- [ ] data validation
- [x] make cell-level modifiers override row-level
- [x] data validation
- [x] fonts
- [x] freeze
- [x] vertical alignment
- [x] hyperlinks
- [x] note
- [x] border options
- [x] ranges that rows can apply expand to
- [x] horizontal alignment
- [x] formatting: bold, italic, underlinen

# Variables and functions

- [ ] a way to not show a function if it's dependent cells are blank (make it less ugly when expanded out)
- [ ] CELLREF()
- [ ] SHEETREF()
- [x] user defined functions at the top
- [x] `$$row` current row

# Performance

- [ ] concurrently process each cell?
- [ ] memoization and lazy evaluation as much as possible
- [ ] matrix libs?

# Other

- [ ] Revise the "how to use" docs
- [ ] Charts
- [ ] Pivot table support - not sure what this would even look like
- [ ] backups before each write?
- [x] Benchmarks in verbose mode
- [x] Simplecov
- [X] Rubocop
- [x] create the spreadsheet if it doesn't exist
- [x] verbose mode
