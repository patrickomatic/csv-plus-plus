# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Provides +Runtime::Value+s for builtin functions and variables
    module Builtins
      extend ::T::Sig

      extend ::CSVPlusPlus::Entities::ASTBuilder

      VARIABLES = ::T.let(
        {
          # The number (integer) of the current cell. Starts at 1
          cellnum: ::CSVPlusPlus::Runtime::Value.new(->(r, _args) { number(r.cell_index + 1) }),

          # A reference to the current cell
          cellref: ::CSVPlusPlus::Runtime::Value.new(
            lambda { |r, _args|
              cell_reference(row_index: r.row_index, cell_index: r.cell_index)
            }
          ),

          # A reference to the row above
          rowabove: ::CSVPlusPlus::Runtime::Value.new(
            lambda { |r, _args|
              cell_reference(row_index: [0, (r.row_index - 1)].max)
            }
          ),

          # A reference to the row below
          rowbelow: ::CSVPlusPlus::Runtime::Value.new(->(r, _args) { cell_reference(row_index: r.row_index + 1) }),

          # The number (integer) of the current row.  Starts at 1
          rownum: ::CSVPlusPlus::Runtime::Value.new(->(r, _args) { number(r.rownum) }),

          # A reference to the current row
          rowref: ::CSVPlusPlus::Runtime::Value.new(->(r, _args) { cell_reference(row_index: r.row_index) })
        }.freeze,
        ::T::Hash[::Symbol, ::CSVPlusPlus::Runtime::Value]
      )
      public_constant :VARIABLES

      # TODO: A reference to a cell in a given row?
      # TODO: check types of the args and throw a more friendly error?
      FUNCTIONS = ::T.let(
        {
          # A reference to a cell above the current row
          cellabove: ::CSVPlusPlus::Runtime::Value.new(
            lambda { |r, args|
              cell_reference(cell_index: args[0].cell_index, row_index: [0, (r.row_index - 1)].max)
            }
          ),

          # A reference to a cell in the current row
          celladjacent: ::CSVPlusPlus::Runtime::Value.new(
            lambda { |r, args|
              cell_reference(cell_index: args[0].cell_index, row_index: r.row_index)
            }
          ),

          # A reference to a cell below the current row
          cellbelow: ::CSVPlusPlus::Runtime::Value.new(
            lambda { |r, args|
              cell_reference(cell_index: args[0].cell_index, row_index: r.row_index + 1)
            }
          )
        }.freeze,
        ::T::Hash[::Symbol, ::CSVPlusPlus::Runtime::Value]
      )
      public_constant :FUNCTIONS
    end
  end
end
