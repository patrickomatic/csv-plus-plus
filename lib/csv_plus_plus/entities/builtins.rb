# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Provides +RuntimeValue+s for builtin functions and variables
    module Builtins
      extend ::T::Sig

      extend ::CSVPlusPlus::Entities::ASTBuilder

      VARIABLES = ::T.let(
        {
          # The number (integer) of the current cell. Starts at 1
          cellnum: runtime_value(->(p, _args) { number(p.cell_index + 1) }),

          # A reference to the current cell
          cellref: runtime_value(->(p, _args) { cell_ref(p.row_index, p.cell_index) }),

          # A reference to the row above
          rowabove: runtime_value(->(p, _args) { cell_ref([0, (p.row_index - 1)].max) }),

          # A reference to the row below
          rowbelow: runtime_value(->(p, _args) { cell_ref(p.row_index + 1) }),

          # The number (integer) of the current row.  Starts at 1
          rownum: runtime_value(->(p, _args) { number(p.rownum) }),

          # A reference to the current row
          rowref: runtime_value(->(p, _args) { cell_ref(p.row_index) })
        }.freeze,
        ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::RuntimeValue]
      )
      public_constant :VARIABLES

      # TODO: A reference to a cell in a given row?
      # TODO: check types of the args and throw a more friendly error?
      FUNCTIONS = ::T.let(
        {
          # A reference to a cell above the current row
          cellabove: runtime_value(->(p, args) { cell_ref([0, (p.row_index - 1)].max, args[0].a1_ref.cell_index) }),

          # A reference to a cell in the current row
          celladjacent: runtime_value(->(p, args) { cell_ref(p.row_index, args[0].a1_ref.cell_index) }),

          # A reference to a cell below the current row
          cellbelow: runtime_value(->(p, args) { cell_ref(p.row_index + 1, args[0].a1_ref.cell_index) })
        }.freeze,
        ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::RuntimeValue]
      )
      public_constant :FUNCTIONS

      sig { params(fn_id: ::Symbol).returns(::T::Boolean) }
      # Is +fn_id+ a builtin function?
      #
      # @param fn_id [Symbol] The Function#id to check if it's a runtime variable
      #
      # @return [T::Boolean]
      def self.builtin_function?(fn_id)
        ::CSVPlusPlus::Entities::Builtins::FUNCTIONS.key?(fn_id)
      end

      sig { params(var_id: ::Symbol).returns(::T::Boolean) }
      # Is +var_id+ a builtin variable?
      #
      # @param var_id [Symbol] The Variable#id to check if it's a runtime variable
      #
      # @return [Boolean]
      def self.builtin_variable?(var_id)
        ::CSVPlusPlus::Entities::Builtins::VARIABLES.key?(var_id)
      end

      sig do
        params(row_index: ::Integer, cell_index: ::T.nilable(::Integer)).returns(::CSVPlusPlus::Entities::Reference)
      end
      # @param row_index [Integer]
      # @param cell_index [Integer, nil]
      #
      # @return [Runtime::Reference]
      def self.cell_ref(row_index, cell_index = nil)
        ::CSVPlusPlus::Entities::Reference.new(a1_ref: ::CSVPlusPlus::A1Reference.new(row_index:, cell_index:))
      end
      private_class_method :cell_ref
    end
  end
end
