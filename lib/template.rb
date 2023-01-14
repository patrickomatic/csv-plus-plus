# frozen_string_literal: true

require_relative './language/syntax_error'
require_relative 'row'

module CSVPlusPlus
  # Contains the flow and data from a code section and CSV section
  class Template
    attr_reader :code_section, :rows

    # initialize
    def initialize(rows: [], code_section: nil)
      @rows = rows
      @code_section = code_section
    end

    # to_s
    def to_s
      "Template(code_section: #{@code_section}, rows: #{@rows})"
    end

    # Apply any expand= modifiers to the parsed template
    def expand_rows!
      expanded_rows = []
      row_index = 0
      expand_rows(
        lambda do |new_row|
          new_row.index = row_index
          expanded_rows << new_row
          row_index += 1
        end
      )

      @rows = expanded_rows
    end

    # Make sure that the template has a valid amount of infinite expand modifiers
    def validate_infinite_expands(runtime)
      infinite_expand_rows = @rows.filter { |r| r.modifier.expand&.infinite? }
      return unless infinite_expand_rows.length > 1

      runtime.raise_syntax_error(
        'You can only have one infinite expand= (on all others you must specify an amount)',
        infinite_expand_rows[1]
      )
    end

    private

    def expand_rows(push_row_fn)
      # TODO: make it so that an infinite expand will not overwrite the rows below it, but
      # instead merge with them
      rows.each do |row|
        if row.modifier.expand
          row.expand_amount.times do
            push_row_fn.call(row.deep_clone)
          end
        else
          push_row_fn.call(row)
        end
      end
    end
  end
end
