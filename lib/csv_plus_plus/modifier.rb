# frozen_string_literal: true

require 'set'
require_relative './color'
require_relative './expand'
require_relative './language/syntax_error'

module CSVPlusPlus
  # A container representing the operations that can be applied to a cell or row
  #
  # @attr expand [Expand]
  # @attr fontfamily [String]
  # @attr fontsize [String]
  # @attr halign ['left', 'center', 'right']
  # @attr valign ['top', 'center', 'bottom']
  # @attr note [String]
  # @attr numberformat [String]
  # @attr row_level [Boolean]
  # @attr validation [Object]
  #
  # @attr_writer borderstyle [String]
  #
  # @attr_reader bordercolor [String]
  # @attr_reader borders [Array<String>]
  # @attr_reader color [Color]
  # @attr_reader fontcolor [Color]
  # @attr_reader formats [Array<String>]
  class Modifier
    attr_reader :bordercolor, :borders, :color, :fontcolor, :formats
    attr_writer :borderstyle
    attr_accessor :expand, :fontfamily, :fontsize, :halign, :valign, :note, :numberformat, :row_level, :validation

    # @param row_level [Boolean] Whether or not this modifier applies to the entire row
    def initialize(row_level: false)
      @row_level = row_level
      @freeze = false
      @borders = ::Set.new
      @formats = ::Set.new
    end

    # Set the color
    # @param hex_value [String]
    def color=(hex_value)
      @color = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Assign a border
    # @param side ['top', 'left', 'bottom', 'right', 'all']
    def border=(side)
      @borders << side
    end

    # Does this have a border along +side+?
    # @param side ['top', 'left', 'bottom', 'right', 'all']
    # @return [Boolean]
    def border_along?(side)
      @borders.include?('all') || @borders.include?(side)
    end

    # Does this have a border along all sides?
    # @return [Boolean]
    def border_all?
      @borders.include?('all') \
        || (border_along?('top') && border_along?('bottom') && border_along?('left') && border_along?('right'))
    end

    # Set the bordercolor
    # @param hex_value [String] formatted as '#000000', '#000' or '000000'
    def bordercolor=(hex_value)
      @bordercolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Are there any borders set?
    # @return [Boolean]
    def any_border?
      !@borders.empty?
    end

    # Set the fontcolor
    # @param hex_value [String] formatted as '#000000', '#000' or '000000'
    def fontcolor=(hex_value)
      @fontcolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Set a text format (bolid, italic, underline or strikethrough)
    # @param value ['bold', 'italic', 'underline', 'strikethrough']
    def format=(value)
      @formats << value
    end

    # Is the given format set?
    # @param type ['bold', 'italic', 'underline', 'strikethrough']
    # @return [Boolean]
    def formatted?(type)
      @formats.include?(type)
    end

    # Freeze the row from edits
    def freeze!
      @frozen = true
    end

    # Is the row forzen?
    # @return [Boolean]
    def frozen?
      @frozen
    end

    # Mark this modifer as row-level
    def row_level!
      @row_level = true
    end

    # Is this a row-level modifier?
    # @return [Boolean]
    def row_level?
      @row_level
    end

    # Is this a cell-level modifier?
    # @return [Boolean]
    def cell_level?
      !@row_level
    end

    # Style of border
    # @return [String]
    def borderstyle
      @borderstyle || 'solid'
    end

    # @return [String]
    def to_s
      # TODO... I dunno, not sure how to manage this
      "Modifier(row_level: #{@row_level} halign: #{@halign} valign: #{@valign} format: #{@formats} " \
        "font_size: #{@font_size})"
    end

    # Create a new modifier instance, with all values defaulted from +other+
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
