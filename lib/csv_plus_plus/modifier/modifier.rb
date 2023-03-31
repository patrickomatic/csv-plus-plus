# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # A container representing the operations that can be applied to a cell or row
    #
    # @attr bordercolor [Color]
    # @attr borders [Array<Symbol>] The borders that will be set
    # @attr color [Color] The background color of the cell
    # @attr expand [Expand] Whether this row expands into multiple rows
    # @attr fontcolor [Color] The font color of the cell
    # @attr fontfamily [::String] The font family
    # @attr fontsize [Integer] The font size
    # @attr halign [:left, :center, :right] Horizontal alignment
    # @attr note [::String] A note/comment on the cell
    # @attr numberformat [Symbol] A number format to apply to the value in the cell
    # @attr row_level [boolean] Is this a row modifier? If so it's values will apply to all cells in the row
    #   (unless overridden by the cell modifier)
    # @attr validation [Object]
    # @attr valign [:top, :center, :bottom] Vertical alignment
    # @attr var [Symbol] The variable bound to this cell
    #
    # @attr_writer borderstyle [:hashed, :dotted, :double, :solid, :solid_medium, :solid_thick]
    #   The style of border on the cell
    #
    # @attr_reader borders [Array<Symbol>]
    # @attr_reader formats [Array<Symbol>] Bold/italics/underline/strikethrough formatting
    class Modifier
      attr_accessor :bordercolor,
                    :color,
                    :expand,
                    :fontcolor,
                    :fontfamily,
                    :fontsize,
                    :halign,
                    :valign,
                    :note,
                    :numberformat,
                    :row_level,
                    :validation,
                    :var
      attr_reader :borders, :formats
      attr_writer :borderstyle

      # @param row_level [Boolean] Whether or not this modifier applies to the entire row
      def initialize(row_level: false)
        @row_level = row_level
        @freeze = false
        @borders = ::Set.new
        @formats = ::Set.new
      end

      # Are there any borders set?
      #
      # @return [Boolean]
      def any_border?
        !@borders.empty?
      end

      # Style of border
      #
      # @return [:hashed, :dotted, :double, :solid, :solid_medium, :solid_thick]
      def borderstyle
        @borderstyle || :solid
      end

      # Is this a cell-level modifier?
      #
      # @return [boolean]
      def cell_level?
        !@row_level
      end

      # Assign a border
      #
      # @param side [:top, :left, :bottom, :right, :all]
      def border=(side)
        @borders << side
      end

      # Does this have a border along +side+?
      #
      # @param side [:top, :left, :bottom, :right, :all]
      #
      # @return [boolean]
      def border_along?(side)
        @borders.include?(:all) || @borders.include?(side)
      end

      # Does this have a border along all sides?
      #
      # @return [boolean]
      def border_all?
        @borders.include?(:all) \
          || (border_along?(:top) && border_along?(:bottom) && border_along?(:left) && border_along?(:right))
      end

      # Set this modifier to expand infinitely
      #
      # @return [::Expand, nil]
      def expand!
        @expand = ::CSVPlusPlus::Expand.new if row_level?
      end

      # Set a text format (bolid, italic, underline or strikethrough)
      #
      # @param value [:bold, :italic, :underline, :strikethrough]
      def format=(value)
        @formats << value
      end

      # Is the given format set?
      #
      # @param type [:bold, :italic, :underline, :strikethrough]
      #
      # @return [boolean]
      def formatted?(type)
        @formats.include?(type)
      end

      # Freeze the row from edits
      #
      # @return [true]
      def freeze!
        @frozen = true
      end

      # Is the row frozen?
      #
      # @return [boolean]
      def frozen?
        @frozen
      end

      # Mark this modifer as row-level
      #
      # @return [true]
      def row_level!
        @row_level = true
      end

      # Is this a row-level modifier?
      #
      # @return [boolean]
      def row_level?
        @row_level
      end

      # Create a new modifier instance, with all values defaulted from +other+
      #
      # @param other [Modifier]
      def take_defaults_from!(other)
        other.instance_variables.each do |property|
          # don't propagate row-specific values
          next if property == :@row_level

          value = other.instance_variable_get(property)
          instance_variable_set(property, value.clone)
        end
      end
    end
  end
end
