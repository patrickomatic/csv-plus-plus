# frozen_string_literal: true

module CSVPlusPlus
  # A container representing the operations that can be applied to a cell or row
  #
  # @attr borders [Array<String>] The borders that will be set
  # @attr expand [Expand] Whether this row expands into multiple rows
  # @attr fontfamily [String] The font family
  # @attr fontsize [Number] The font size
  # @attr halign ['left', 'center', 'right'] Horizontal alignment
  # @attr note [String] A note/comment on the cell
  # @attr numberformat [String] A number format to apply to the value in the cell
  # @attr row_level [Boolean] Is this a row modifier? If so it's values will apply to all cells in the row
  #   (unless overridden by the cell modifier)
  # @attr validation [Object]
  # @attr valign ['top', 'center', 'bottom'] Vertical alignment
  #
  # @attr_writer borderstyle ['dashed', 'dotted', 'double', 'solid', 'solid_medium', 'solid_thick']
  #   The style of border on the cell
  #
  # @attr_reader bordercolor [String]
  # @attr_reader borders [Array<String>]
  # @attr_reader color [Color] The background color of the cell
  # @attr_reader fontcolor [Color] The font color of the cell
  # @attr_reader formats [Array<String>] Bold/italics/underline/strikethrough formatting
  class Modifier
    attr_reader :bordercolor, :borders, :color, :fontcolor, :formats, :variable
    attr_writer :borderstyle
    attr_accessor :expand, :fontfamily, :fontsize, :halign, :valign, :note, :numberformat, :row_level, :validation

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
    # @return [String]
    def borderstyle
      @borderstyle || 'solid'
    end

    # Is this a cell-level modifier?
    #
    # @return [Boolean]
    def cell_level?
      !@row_level
    end

    # Set the color
    #
    # @param hex_value [String]
    #
    # @return [Color]
    def color=(hex_value)
      @color = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Assign a border
    #
    # @param side ['top', 'left', 'bottom', 'right', 'all']
    def border=(side)
      @borders << side
    end

    # Does this have a border along +side+?
    #
    # @param side ['top', 'left', 'bottom', 'right', 'all']
    #
    # @return [Boolean]
    def border_along?(side)
      @borders.include?('all') || @borders.include?(side)
    end

    # Does this have a border along all sides?
    #
    # @return [Boolean]
    def border_all?
      @borders.include?('all') \
        || (border_along?('top') && border_along?('bottom') && border_along?('left') && border_along?('right'))
    end

    # Set the bordercolor
    #
    # @param hex_value [String] formatted as '#000000', '#000' or '000000'
    def bordercolor=(hex_value)
      @bordercolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Set the fontcolor
    #
    # @param hex_value [String] formatted as '#000000', '#000' or '000000'
    def fontcolor=(hex_value)
      @fontcolor = ::CSVPlusPlus::Color.new(hex_value)
    end

    # Set a text format (bolid, italic, underline or strikethrough)
    #
    # @param value ['bold', 'italic', 'underline', 'strikethrough']
    def format=(value)
      @formats << value
    end

    # Is the given format set?
    #
    # @param type ['bold', 'italic', 'underline', 'strikethrough']
    #
    # @return [Boolean]
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
    # @return [Boolean]
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
    # @return [Boolean]
    def row_level?
      @row_level
    end

    # @return [String]
    def to_s
      # TODO... I dunno, not sure how to manage this
      "Modifier(row_level: #{@row_level} halign: #{@halign} valign: #{@valign} format: #{@formats} " \
        "font_size: #{@font_size})"
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

    # Bind this cell to a variable
    def variable=(variable_id)
      @variable = ::CSVPlusPlus::Language::Entities::CellReference.new(variable_id)
    end
  end
end
