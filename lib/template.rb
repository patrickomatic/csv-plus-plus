require 'csv'
require_relative 'row'
require_relative 'code_section'
require_relative './language/syntax_error'
require_relative './language/execution_context'

module CSVPlusPlus
  class Template
    attr_reader :code_section, :rows

    def initialize(rows: [], code_section: nil, execution_context:)
      @rows = rows
      @code_section = code_section
      @ec = execution_context
    end

    def self.run(execution_context:, key_values: {})
      Template.new(execution_context:).tap do |t|
        t.parse_code_section! key_values
        t.parse_csv_rows!
        t.expand_rows!
        t.resolve_variables!
        t.apply_functions!
      end
    end

    def to_s
      "Template(key_values: #{@key_values.to_s}, rows: #{@rows.to_s}, execution_context: #{ec})"
    end

    def parse_code_section!(key_values)
      @code_section = CodeSection.parse(@ec)
    end

    def parse_csv_rows!
      @ec.parsing_csv! do |input|
        infinite_expand = nil

        @rows = @ec.map_rows(CSV.new input) do |csv_row|
          Row.parse(csv_row, @ec).tap do |r|
            if r.modifier.expand&.infinite?
              if infinite_expand
                raise Language::SyntaxError.new(
                  'You can only have one infinite expand= (on all others you must specify an amount)',
                  csv_row,
                  @ec)
              else
                infinite_expand = r.modifier
              end
            end
          end
        end
      end
    end

    def expand_rows!
      @ec.expanding! do
        expanded_rows, row_index = [], 0

        push_row = ->(new_row) do
          new_row.index = row_index
          expanded_rows << new_row
          row_index += 1
        end

        # TODO make it so that an infinite expand will not overwrite the rows below it, but
        # instead merge with them
        @ec.map_rows(@rows) do |row|
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

    def resolve_variables!
      @ec.resolve_all_cells!(@code_section, @rows)
    end

    def apply_functions!
      @ec.applying_functions! do
      # XXX do a DFS, replacing each function (just builtins for now) with their AST, with the
      # args interpolated
      end
    end
  end
end
