# frozen_string_literal: true

require 'csv'
require_relative './language/compiler'
require_relative './language/syntax_error'
require_relative 'row'

module CSVPlusPlus
  # Contains the flow and data from a code section and CSV section
  class Template
    attr_reader :code_section, :rows

    # Run the full lifecycle of a template
    def self.run(compiler:, key_values: {})
      new(compiler:).tap do |t|
        t.parse_code_section!(key_values)
        t.parse_csv_rows!
        t.expand_rows!
        t.resolve_variables!
        t.apply_functions!
      end
    end

    # initialize
    def initialize(compiler:, rows: [], code_section: nil)
      @rows = rows
      @code_section = code_section
      @compiler = compiler
    end

    # to_s
    def to_s
      "Template(key_values: #{@key_values}, rows: #{@rows}, compiler: #{@compiler})"
    end

    # Parse the code section of the template
    def parse_code_section!(key_values)
      @code_section = @compiler.parse_code_section(key_values:)
    end

    # Parse the CSV section of the template
    def parse_csv_rows!
      @compiler.parsing_csv! do |input|
        @rows =
          @compiler.map_rows(::CSV.new(input)) do |csv_row|
            ::CSVPlusPlus::Row.parse(csv_row, @compiler)
          end

        validate_infinite_expands
      end
    end

    # Apply any expand= modifiers to the parsed template
    # rubocop:todo Metrics/MethodLength
    def expand_rows!
      @compiler.expanding! do
        expanded_rows = []
        row_index = 0

        push_row =
          lambda do |new_row|
            new_row.index = row_index
            expanded_rows << new_row
            row_index += 1
          end

        # TODO: make it so that an infinite expand will not overwrite the rows below it, but
        # instead merge with them
        @compiler.map_rows(@rows) do |row|
          if row.modifier.expand
            row.expand_amount.times do
              push_row.call(row.deep_clone)
            end
          else
            push_row.call(row)
          end
        end

        @rows = expanded_rows
      end
    end
    # rubocop:enable Metrics/MethodLength

    # Apply any runtime or user-supplied variables to the individual cell values
    def resolve_variables!
      @compiler.resolve_all_cells!(@code_section, @rows)
    end

    # Apply any runtime or user-supplied functions  to the individual cell values
    def apply_functions!
      @compiler.applying_functions! do
        # XXX do a DFS, replacing each function (just builtins for now) with their AST, with the
        # args interpolated
      end
    end

    private

    def validate_infinite_expands
      infinite_expand_rows = @rows.filter { |r| r.modifier.expand&.infinite? }
      return unless infinite_expand_rows.length > 1

      raise(
        ::CSVPlusPlus::Language::SyntaxError.new(
          'You can only have one infinite expand= (on all others you must specify an amount)',
          infinite_expand_rows[1],
          @compiler
        )
      )
    end
  end
end
