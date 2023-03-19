# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Provides ASTs for builtin functions and variables
    module Builtins
      extend ::CSVPlusPlus::Entities::ASTBuilder

      VARIABLES = {
        # The number (integer) of the current cell. Starts at 1
        cellnum: runtime_value(->(runtime) { number(runtime.cell_index + 1) }),

        # A reference to the current cell
        cellref: runtime_value(->(runtime) { ref(row_index: runtime.row_index, cell_index: runtime.cell_index) }),

        # A reference to the row above
        rowabove: runtime_value(->(runtime) { ref(row_index: [0, (runtime.row_index - 1)].max) }),

        # A reference to the row below
        rowbelow: runtime_value(->(runtime) { ref(row_index: runtime.row_index + 1) }),

        # The number (integer) of the current row.  Starts at 1
        rownum: runtime_value(->(runtime) { number(runtime.rownum) }),

        # A reference to the current row
        rowref: runtime_value(->(runtime) { ref(row_index: runtime.row_index) })
      }.freeze
      public_constant :VARIABLES

      FUNCTIONS = {
        # TODO: A reference to a cell in a given row?
        # A reference to a cell above the current row
        cellabove: runtime_value(->(runtime, args) { cell_reference([args[0], [1, (runtime.rownum - 1)].max].join) }),

        # A reference to a cell in the current row
        celladjacent: runtime_value(->(runtime, args) { cell_reference([args[0], runtime.rownum].join) }),

        # A reference to a cell below the current row
        cellbelow: runtime_value(->(runtime, args) { cell_reference([args[0], runtime.rownum + 1].join) })
      }.freeze
      public_constant :FUNCTIONS
    end
  end
end
