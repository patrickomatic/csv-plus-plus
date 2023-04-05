# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # A container representing the operations that can be applied to a cell or row
    #
    # @attr bordercolor [Color, nil]
    # @attr color [Color, nil] The background color of the cell
    # @attr expand [Modifier::Expand, nil] Whether this row expands into multiple rows
    # @attr fontcolor [Color, nil] The font color of the cell
    # @attr fontfamily [::String, nil] The font family
    # @attr fontsize [Numeric, nil] The font size
    # @attr halign [Modifier::HorizontalAlign, nil] Horizontal alignment
    # @attr note [::String, nil] A note/comment on the cell
    # @attr numberformat [Modifier::NumberFormat, nil] A number format to apply to the value in the cell
    # @attr row_level [T::Boolean] Is this a row modifier? If so it's values will apply to all cells in the row
    #   (unless overridden by the cell modifier)
    # @attr validate [Modifier::DataValidation, nil]
    # @attr valign [Modifier::VerticalAlign, nil] Vertical alignment
    # @attr var [Symbol, nil] The variable bound to this cell
    #
    # @attr_writer borderstyle [Modifier::BorderStyle] The style of border on the cell
    #
    # @attr_reader borders [Set<Modifier::BorderSide>] The sides of the cell where a border will be applied.
    # @attr_reader formats [Set<Modifier::TextFormat>] Bold/italics/underline and strikethrough formatting.
    #
    # rubocop:disable Metrics/ClassLength
    class Modifier
      extend ::T::Sig

      sig { returns(::T.nilable(::CSVPlusPlus::Color)) }
      attr_accessor :bordercolor

      sig { returns(::T.nilable(::CSVPlusPlus::Color)) }
      attr_accessor :color

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::Expand)) }
      attr_accessor :expand

      sig { returns(::T.nilable(::CSVPlusPlus::Color)) }
      attr_accessor :fontcolor

      sig { returns(::T.nilable(::String)) }
      attr_accessor :fontfamily

      sig { returns(::T.nilable(::Numeric)) }
      attr_accessor :fontsize

      sig { returns(::T::Boolean) }
      attr_accessor :frozen

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::HorizontalAlign)) }
      attr_accessor :halign

      sig { returns(::T.nilable(::String)) }
      attr_accessor :note

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::NumberFormat)) }
      attr_accessor :numberformat

      sig { returns(::T::Boolean) }
      attr_accessor :row_level

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::DataValidation)) }
      attr_accessor :validate

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::VerticalAlign)) }
      attr_accessor :valign

      sig { returns(::T.nilable(::Symbol)) }
      attr_accessor :var

      sig { returns(::T::Set[::CSVPlusPlus::Modifier::BorderSide]) }
      attr_reader :borders

      sig { returns(::T::Set[::CSVPlusPlus::Modifier::TextFormat]) }
      attr_reader :formats

      sig do
        params(borderstyle: ::CSVPlusPlus::Modifier::BorderStyle)
          .returns(::T.nilable(::CSVPlusPlus::Modifier::BorderStyle))
      end
      attr_writer :borderstyle

      sig { params(row_level: ::T::Boolean).void }
      # @param row_level [Boolean] Whether or not this modifier applies to the entire row
      def initialize(row_level: false)
        @row_level = row_level
        @frozen = ::T.let(false, ::T::Boolean)
        @borders = ::T.let(::Set.new, ::T::Set[::CSVPlusPlus::Modifier::BorderSide])
        @formats = ::T.let(::Set.new, ::T::Set[::CSVPlusPlus::Modifier::TextFormat])
      end

      sig { returns(::T::Boolean) }
      # Are there any borders set?
      #
      # @return [::T::Boolean]
      def any_border?
        !@borders.empty?
      end

      sig { returns(::CSVPlusPlus::Modifier::BorderStyle) }
      # Style of the border
      #
      # @return [::CSVPlusPlus::Modifier::BorderStyle]
      def borderstyle
        @borderstyle || ::CSVPlusPlus::Modifier::BorderStyle::Solid
      end

      sig { returns(::T::Boolean) }
      # Is this a cell-level modifier?
      #
      # @return [T::Boolean]
      def cell_level?
        !@row_level
      end

      sig { params(side: ::CSVPlusPlus::Modifier::BorderSide).returns(::T::Set[::CSVPlusPlus::Modifier::BorderSide]) }
      # Put a border on the given +side+
      #
      # @param side [Modifier::BorderSide]
      def border=(side)
        @borders << side
      end

      sig { params(side: ::CSVPlusPlus::Modifier::BorderSide).returns(::T::Boolean) }
      # Does this have a border along +side+?
      #
      # @param side [Modifier::BorderSide]
      #
      # @return [T::Boolean]
      def border_along?(side)
        @borders.include?(::CSVPlusPlus::Modifier::BorderSide::All) || @borders.include?(side)
      end

      sig { returns(::T::Boolean) }
      # Does this have a border along all sides?
      #
      # @return [T::Boolean]
      def border_all?
        @borders.include?(::CSVPlusPlus::Modifier::BorderSide::All) \
          || (border_along?(::CSVPlusPlus::Modifier::BorderSide::Top) \
              && border_along?(::CSVPlusPlus::Modifier::BorderSide::Bottom) \
              && border_along?(::CSVPlusPlus::Modifier::BorderSide::Left) \
              && border_along?(::CSVPlusPlus::Modifier::BorderSide::Right))
      end

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::Expand)) }
      # Set this modifier to expand infinitely
      #
      # @return [Expand, nil]
      def infinite_expand!
        @expand = ::CSVPlusPlus::Modifier::Expand.new if row_level?
      end

      sig { params(format: ::CSVPlusPlus::Modifier::TextFormat).returns(::T::Set[::CSVPlusPlus::Modifier::TextFormat]) }
      # Set a text format (bolid, italic, underline or strikethrough)
      #
      # @param value [TextFormat]
      def format=(format)
        @formats << format
      end

      sig { params(format: ::CSVPlusPlus::Modifier::TextFormat).returns(::T::Boolean) }
      # Is the given format set?
      # @param format [TextFormat]
      #
      # @return [T::Boolean]
      def formatted?(format)
        @formats.include?(format)
      end

      sig { returns(::T::Boolean) }
      # Freeze the row from edits
      #
      # @return [true]
      def freeze!
        @frozen = true
      end

      sig { returns(::T::Boolean) }
      # Mark this modifer as row-level
      #
      # @return [true]
      def row_level!
        @row_level = true
      end

      sig { returns(::T::Boolean) }
      # Is this a row-level modifier?
      #
      # @return [T::Boolean]
      def row_level?
        @row_level
      end

      sig { params(other: ::CSVPlusPlus::Modifier::Modifier).void }
      # Create a new modifier instance, with all values defaulted from +other+
      #
      # @param other [Modifier]
      #
      # @return [Modifier]
      def take_defaults_from!(other)
        other.instance_variables.each do |property|
          # don't propagate row-specific values
          next if property == :@row_level

          value = other.instance_variable_get(property)
          instance_variable_set(property, value.clone)
        end
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end
