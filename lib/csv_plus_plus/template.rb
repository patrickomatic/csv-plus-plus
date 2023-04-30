# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Contains the data from a parsed csvpp template.
  #
  # @attr_reader rows [Array<Row>] The +Row+s that comprise this +Template+
  # @attr_reader runtime [Runtime] The +Runtime+ containing all function and variable references
  class Template
    extend ::T::Sig

    sig { returns(::T::Array[::CSVPlusPlus::Row]) }
    attr_reader :rows

    sig { returns(::CSVPlusPlus::Runtime::Runtime) }
    attr_reader :runtime

    sig { params(rows: ::T::Array[::CSVPlusPlus::Row], runtime: ::CSVPlusPlus::Runtime::Runtime).void }
    # @param rows [Array<Row>] The +Row+s that comprise this +Template+
    # @param runtime [Runtime] The +Runtime+ containing all function and variable references
    def initialize(rows:, runtime:)
      @rows = rows
      @runtime = runtime
    end

    sig { params(runtime: ::CSVPlusPlus::Runtime::Runtime).void }
    # Only run after expanding all rows, now we can bind all [[var=]] modifiers to a variable.  There are two distinct
    # types of variable bindings here:
    #
    # * Binding to a cell: for this we just make an +A1Reference+ to the cell itself (A1, B4, etc)
    # * Binding to a cell within an expand: the variable can only be resolved within that expand and needs to be
    #   relative to it's row (it can't be an absolute cell reference like above)
    #
    # @param runtime [Runtime] The current runtime
    # rubocop:disable Metrics/MethodLength
    def bind_all_vars!(runtime)
      runtime.position.map_rows(@rows) do |row|
        # rubocop:disable Style/MissingElse
        if row.unexpanded?
          # rubocop:enable Style/MissingElse
          raise(
            ::CSVPlusPlus::Error::CompilerError,
            'Template#expand_rows! must be called before Template#bind_all_vars!'
          )
        end

        runtime.position.map_row(row.cells) do |cell|
          bind_vars(cell, row.modifier.expand)
        end
      end
    end
    # rubocop:enable Metrics/MethodLength

    sig { returns(::T::Array[::CSVPlusPlus::Row]) }
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

    sig { void }
    # Make sure that the template has a valid amount of infinite expand modifiers
    def validate_infinite_expands
      infinite_expand_rows = @rows.filter { |r| r.modifier.expand&.infinite? }
      return unless infinite_expand_rows.length > 1

      raise(
        ::CSVPlusPlus::Error::ModifierSyntaxError.new(
          'You can only have one infinite expand= (on all others you must specify an amount)',
          bad_input: infinite_expand_rows[1].to_s
        )
      )
    end

    sig { returns(::String) }
    # Provide a summary of the state of the template (and it's +@runtime+)
    #
    # @return [::String]
    def verbose_summary
      # TODO: we can probably include way more stats in here
      <<~SUMMARY
        #{@runtime.scope.verbose_summary}

        > #{@rows.length} rows to be written
      SUMMARY
    end

    private

    sig { params(cell: ::CSVPlusPlus::Cell, expand: ::T.nilable(::CSVPlusPlus::Modifier::Expand)).void }
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
