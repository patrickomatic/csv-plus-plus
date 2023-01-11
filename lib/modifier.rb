# frozen_string_literal: true

require 'set'
require_relative './color'
require_relative './expand'
require_relative './language/syntax_error'

module CSVPlusPlus
  ##
  # A container representing the operations that can be applied to a cell or row
  class Modifier
    attr_reader :bordercolor, :borders, :color, :fontcolor, :formats
    attr_accessor :expand, :fontfamily, :fontsize, :note, :numberformat, :row_level, :validation

    # initialize
    def initialize(row_level: false)
      @row_level = row_level
      @freeze = false
      @align = ::Set.new
      @borders = ::Set.new
      @formats = ::Set.new
    end

    # Set an align format. +direction+ must be 'center', 'left', 'right', 'bottom'
    def align=(direction)
      @align << direction
    end

    # Is it aligned to a given direction?
    def aligned?(direction)
      @align.include?(direction)
    end

    # Set the color.  hex_value is a String
    def color=(hex_value)
      @color = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Assign a border.  +side+ must be 'top', 'left', 'bottom', 'right' or 'all'
    def border=(side)
      @borders << side
    end

    # Does this have a border along +side+?
    def border_along?(side)
      border_all? || @borders.include?(side)
    end

    # Does this have a border along all sides?
    def border_all?
      @borders.include?('all')
    end

    # Set the bordercolor
    def bordercolor=(hex_value)
      @bordercolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Are there any borders set?
    def any_border?
      !@borders.empty?
    end

    # Set the fontcolor
    def fontcolor=(hex_value)
      @fontcolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Set a format.  +type+ must be 'bold', 'italic', 'underline' or 'strikethrough'
    def format=(value)
      @formats << value
    end

    # Is the given format set?
    def formatted?(type)
      @formats.include?(type)
    end

    # Freeze the row from edits
    def freeze!
      @frozen = true
    end

    # Is the row forzen?
    def frozen?
      @frozen
    end

    # Mark this modifer as row-level
    def row_level!
      @row_level = true
    end

    # Is this a row-level modifier?
    def row_level?
      @row_level
    end

    # Is this a cell-level modifier?
    def cell_level?
      !@row_level
    end

    # Style of border
    def borderstyle
      @borderstyle || 'solid'
    end

    # to_s
    def to_s
      # TODO... I dunno, not sure how to manage this
      "Modifier(row_level: #{@row_level} align: #{@align} format: #{@formats} font_size: #{@font_size})"
    end

    # Create a new modifier instance, with all values defaulted from +other+
    # rubocop:disable Metrics/MethodLength
    def take_defaults_from!(other)
      # TODO: can I just infer the instance vars?
      %i[
        @align
        @bordercolor
        @borders
        @borderstyle
        @color
        @formats
        @fontcolor
        @fontfamily
        @fontsize
        @note
        @numberformat
        @validation
      ].each do |property|
        value = other.instance_variable_get(property)
        instance_variable_set(property, value.clone)
      end
    end
    # rubocop:enable Metrics/MethodLength
  end
end
