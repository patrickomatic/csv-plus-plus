# frozen_string_literal: true

module CSVPlusPlus
  # Contains the data from a parsed csvpp template.
  #
  # @attr_reader rows [Array<Row>] The +Row+s that comprise this +Template+
  # @attr_reader scope [Scope] The +Scope+ containing all function and variable references
  class Template
    attr_reader :rows, :scope

    # @param rows [Array<Row>] The +Row+s that comprise this +Template+
    # @param scope [Scope] The +Scope+ containing all function and variable references
    def initialize(rows:, scope:)
      @scope = scope
      @rows = rows
    end

    # @return [String]
    def to_s
      "Template(rows: #{@rows}, scope: #{@scope})"
    end

    # Apply any expand= modifiers to the parsed template
    #
    # @return [Array<Row>]
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
    #
    # @param runtime [Runtime] The compiler's current runtime
    def validate_infinite_expands(runtime)
      infinite_expand_rows = @rows.filter { |r| r.modifier.expand&.infinite? }
      return unless infinite_expand_rows.length > 1

      runtime.raise_formula_syntax_error(
        'You can only have one infinite expand= (on all others you must specify an amount)',
        infinite_expand_rows[1]
      )
    end

    # Provide a summary of the state of the template (and it's +@scope+)
    #
    # @return [String]
    def verbose_summary
      # TODO: we can probably include way more stats in here
      <<~SUMMARY
        #{@scope.verbose_summary}

        > #{@rows.length} rows to be written
      SUMMARY
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
