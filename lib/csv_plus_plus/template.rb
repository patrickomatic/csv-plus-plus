# frozen_string_literal: true

module CSVPlusPlus
  # Contains the data from a parsed csvpp template.
  #
  # @attr_reader rows [Array<Row>] The +Row+s that comprise this +Template+
  # @attr_reader runtime [Runtime] The +Runtime+ containing all function and variable references
  class Template
    attr_reader :rows, :runtime

    # @param rows [Array<Row>] The +Row+s that comprise this +Template+
    # @param runtime [Runtime] The +Runtime+ containing all function and variable references
    def initialize(rows:, runtime:)
      @runtime = runtime
      @rows = rows
    end

    # Only run after expanding all rows, now we can bind all [[var=]] modifiers to a variable.  There are two distinct
    # types of variable bindings here:
    #
    # * Binding to a cell: for this we just make a +CellReference+ to the cell itself (A1, B4, etc)
    # * Binding to a cell within an expand: the variable can only be resolved within that expand and needs to be
    #   relative to it's row (it can't be an absolute cell reference like above)
    #
    # @param runtime [Runtime]
    def bind_all_vars!(runtime)
      runtime.map_rows(@rows) do |row|
        # rubocop:disable Style/MissingElse
        if row.unexpanded?
          # rubocop:enable Style/MissingElse
          raise(::CSVPlusPlus::Error::Error, 'Template#expand_rows! must be called before Template#bind_all_vars!')
        end

        runtime.map_row(row.cells) do |cell|
          bind_vars(cell, row.modifier.expand)
        end
      end
    end

    # Apply expand= (adding rows to the results) modifiers to the parsed template. This happens in towards the end of
    # compilation because expanding rows will change the relative rownums as rows are added, and variables can't be
    # bound until the rows have been assigned their final rownums.
    #
    # @return [Array<Row>]
    def expand_rows!
      # TODO: make it so that an infinite expand will not overwrite the rows below it, but instead merge with them
      @rows =
        rows.reduce([]) do |expanded_rows, row|
          if row.modifier.expand
            row.expand_rows(starts_at: expanded_rows.length, into: expanded_rows)
          else
            expanded_rows << row.tap { |r| r.index = expanded_rows.length }
          end
        end
    end

    # Make sure that the template has a valid amount of infinite expand modifiers
    #
    # @param runtime [Runtime] The compiler's current runtime
    def validate_infinite_expands(runtime)
      infinite_expand_rows = @rows.filter { |r| r.modifier.expand&.infinite? }
      return unless infinite_expand_rows.length > 1

      runtime.raise_modifier_syntax_error(
        'You can only have one infinite expand= (on all others you must specify an amount)',
        infinite_expand_rows[1]
      )
    end

    # Provide a summary of the state of the template (and it's +@runtime+)
    #
    # @return [::String]
    def verbose_summary
      # TODO: we can probably include way more stats in here
      <<~SUMMARY
        #{@runtime.verbose_summary}

        > #{@rows.length} rows to be written
      SUMMARY
    end

    private

    def bind_vars(cell, expand)
      var = cell.modifier.var
      return unless var

      if expand
        @runtime.bind_variable_in_expand(var, expand)
      else
        @runtime.bind_variable_to_cell(var)
      end
    end
  end
end
