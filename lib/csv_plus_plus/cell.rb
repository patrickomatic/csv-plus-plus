# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # A cell of a template
  #
  # @attr ast [Entity]
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
        value: ::T.nilable(::String),
        runtime: ::CSVPlusPlus::Runtime::Runtime,
        modifier: ::CSVPlusPlus::Modifier::Modifier
      ).returns(::CSVPlusPlus::Cell)
    end
    # Parse a +value+ into a Cell object.
    #
    # @param value [String] A string value which should already have been processed through a CSV parser
    # @param runtime [Runtime]
    # @param modifier [Modifier]
    #
    # @return [Cell]
    def self.parse(value, runtime:, modifier:)
      new(value:, row_index: runtime.row_index, index: runtime.cell_index, modifier:).tap do |c|
        c.ast = ::T.unsafe(::CSVPlusPlus::Parser::CellValue.new).parse(value, runtime)
      end
    end

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
    # TODO: is this used?
    def value
      stripped = @value&.strip

      stripped&.empty? ? nil : stripped
    end

    sig { params(runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::T.nilable(::String)) }
    # A compiled final representation of the cell.  This can only happen after all cell have had variables and functions
    # resolved.
    #
    # @param runtime [Runtime]
    #
    # @return [::String]
    def evaluate(runtime)
      return value unless @ast

      "=#{@ast.evaluate(runtime)}"
    end
  end
end
