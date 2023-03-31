# typed: false
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Provides ASTs for builtin functions and variables
    module Builtins
      extend ::CSVPlusPlus::Entities::ASTBuilder

      VARIABLES = {
        # The number (integer) of the current cell. Starts at 1
        cellnum: runtime_value(->(r) { number(r.cell_index + 1) }),

        # A reference to the current cell
        cellref: runtime_value(->(r) { cell_reference(row_index: r.row_index, cell_index: r.cell_index) }),

        # A reference to the row above
        rowabove: runtime_value(->(r) { cell_reference(row_index: [0, (r.row_index - 1)].max) }),

        # A reference to the row below
        rowbelow: runtime_value(->(r) { cell_reference(row_index: r.row_index + 1) }),

        # The number (integer) of the current row.  Starts at 1
        rownum: runtime_value(->(r) { number(r.rownum) }),

        # A reference to the current row
        rowref: runtime_value(->(r) { cell_reference(row_index: r.row_index) })
      }.freeze
      public_constant :VARIABLES

      FUNCTIONS = {
        # TODO: A reference to a cell in a given row?
        # A reference to a cell above the current row
        # cellabove: runtime_value(->(r, args) { cell_reference(ref: [args[0], [1, (r.rownum - 1)].max].join) }),
        cellabove: runtime_value(
          lambda { |r, args|
            cell_reference(cell_index: args[0].cell_index, row_index: [0, (r.row_index - 1)].max)
          }
        ),

        # A reference to a cell in the current row
        celladjacent: runtime_value(
          lambda { |r, args|
            cell_reference(cell_index: args[0].cell_index, row_index: r.row_index)
          }
        ),

        # A reference to a cell below the current row
        cellbelow: runtime_value(
          lambda { |r, args|
            cell_reference(cell_index: args[0].cell_index, row_index: r.row_index + 1)
          }
        )
      }.freeze
      public_constant :FUNCTIONS
    end
  end
end
