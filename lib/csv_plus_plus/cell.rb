# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # A cell of a template
  #
  # @attr ast [Entity, nil] The AST of the formula in the cell (if there is one)
  # @attr row_index [Integer] The cell's row index (starts at 0)
  # @attr_reader index [Integer] The cell's index (starts at 0)
  # @attr_reader modifier [Modifier] The modifier for this cell
  class Cell
    extend ::T::Sig

    sig { returns(::T.nilable(::CSVPlusPlus::Entities::Entity)) }
    attr_accessor :ast

    sig { returns(::Integer) }
    attr_accessor :row_index

    sig { returns(::Integer) }
    attr_reader :index

    sig { returns(::CSVPlusPlus::Modifier::Modifier) }
    attr_reader :modifier

    sig do
      params(
        index: ::Integer,
        modifier: ::CSVPlusPlus::Modifier::Modifier,
        row_index: ::Integer,
        value: ::T.nilable(::String)
      ).void
    end
    # @param index [Integer] The cell's index (starts at 0)
    # @param modifier [Modifier] A modifier to apply to this cell
    # @param row_index [Integer] The cell's row index (starts at 0)
    # @param value [String] A string value which should already have been processed through a CSV parser
    def initialize(index:, modifier:, row_index:, value:)
      @value = value
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    sig { returns(::T.nilable(::String)) }
    # The +@value+ (cleaned up some)
    #
    # @return [::String]
    def value
      stripped = @value&.strip

      stripped&.empty? ? nil : stripped
    end
  end
end
