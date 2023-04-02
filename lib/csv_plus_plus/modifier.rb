# typed: strict
# frozen_string_literal: true

require_relative './modifier/conditional_formatting'
require_relative './modifier/data_validation'
require_relative './modifier/expand'
require_relative './modifier/modifier'
require_relative './modifier/validated_modifier'

module CSVPlusPlus
  # All modifier-specific logic is hidden in this module and callers should just call +#new+ on this module.
  module Modifier
    extend ::T::Sig

    # The sides that a border can be on
    class BorderSide < ::T::Enum
      enums do
        All = new
        Top = new
        Bottom = new
        Left = new
        Right = new
      end
    end

    # The various border styles
    class BorderStyle < ::T::Enum
      enums do
        Dashed = new
        Dotted = new
        Double = new
        Solid = new
        SolidMedium = new
        SolidThick = new
      end
    end

    # The possible values for a horizontal alignment
    class HorizontalAlign < ::T::Enum
      enums do
        Left = new
        Right = new
        Center = new
      end
    end

    # The allowed number formats
    class NumberFormat < ::T::Enum
      enums do
        Currency = new
        Date = new
        DateTime = new
        Number = new
        Percent = new
        Text = new
        Time = new
        Scientific = new
      end
    end

    # The types of formats that can be applied to text.
    class TextFormat < ::T::Enum
      enums do
        Bold = new
        Underline = new
        Strikethrough = new
      end
    end

    # The possible values for a horizontal alignment
    class VerticalAlign < ::T::Enum
      enums do
        Top = new
        Bottom = new
        Center = new
      end
    end

    sig { params(row_level: ::T::Boolean).returns(::CSVPlusPlus::Modifier::Modifier) }
    # Return a +Modifier+ with the proper validation and helper functions attached
    #
    # @param row_level [boolean] is this a row level modifier? (otherwise cell-level)
    #
    # @return [ValidatedModifier]
    def self.new(row_level: false)
      ::CSVPlusPlus::Modifier::ValidatedModifier.new(row_level:)
    end
  end
end
