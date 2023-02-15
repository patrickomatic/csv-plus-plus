# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Build a RubyXL workbook formatted according to the given +rows+
    # rubocop:disable Metrics/ClassLength
    class RubyXLBuilder
      # https://www.rubydoc.info/gems/rubyXL/RubyXL/NumberFormats
      # https://support.microsoft.com/en-us/office/number-format-codes-5026bbd6-04bc-48cd-bf33-80f18b4eae68
      NUM_FMT_IDS = {
        currency: 5,
        date: 14,
        date_time: 22,
        number: 1,
        percent: 9,
        text: 49,
        time: 21,
        scientific: 48
      }.freeze
      private_constant :NUM_FMT_IDS

      # https://www.rubydoc.info/gems/rubyXL/2.3.0/RubyXL
      # ST_BorderStyle = %w{ none thin medium dashed dotted thick double hair mediumDashed dashDot mediumDashDot
      #                      dashDotDot slantDashDot }
      BORDER_STYLES = {
        dashed: 'dashed',
        dotted: 'dotted',
        double: 'double',
        solid: 'thin',
        solid_medium: 'medium',
        solid_thick: 'thick'
      }.freeze
      private_constant :BORDER_STYLES

      attr_reader :output_filename, :rows

      # initialize
      def initialize(rows:, output_filename:)
        @rows = rows
        @output_filename = output_filename
        @workbook = open_workbook
      end

      # write the given @rows in +sheet_name+ to +@output_filename+
      def write(sheet_name)
        # TODO: this is leaving a blank sheet in front of the one we're creating
        @worksheet = @workbook[sheet_name] || @workbook.add_worksheet(sheet_name)
        build_workbook!

        @workbook.write(@output_filename)
      end

      private

      def build_workbook!
        @rows.each_with_index do |row, x|
          row.cells.each_with_index do |cell, y|
            @worksheet.add_cell(x, y, cell.to_csv)
            format_cell!(x, y, cell.modifier)
          end
        end
      end

      # rubocop:disable Metrics/MethodLength, Metrics/PerceivedComplexity
      def do_alignments!(cell, modifier)
        if modifier.aligned?('left')
          cell.change_horizontal_alignment('left')
        elsif modifier.aligned?('right')
          cell.change_horizontal_alignment('right')
          # rubocop:disable Style/MissingElse
        elsif modifier.aligned?('center')
          # rubocop:enable Style/MissingElse
          cell.change_horizontal_alignment('center')
        end

        if modifier.aligned?('top')
          cell.change_vertical_alignment('top')
        elsif modifier.aligned?('bottom')
          cell.change_vertical_alignment('bottom')
          # rubocop:disable Style/MissingElse
        elsif modifier.aligned?('center')
          # rubocop:enable Style/MissingElse
          cell.change_vertical_alignment('center')
        end
      end
      # rubocop:enable Metrics/MethodLength, Metrics/PerceivedComplexity

      def border_weight(modifier)
        # rubocop:disable Lint/ConstantResolution
        BORDER_STYLES[modifier.borderstyle.to_sym]
        # rubocop:enable Lint/ConstantResolution
      end

      # rubocop:disable Metrics/MethodLength
      def do_borders!(cell, modifier)
        return unless modifier.any_border?

        color = modifier.bordercolor
        weight = border_weight(modifier)

        if modifier.border_all?
          %i[top bottom left right].each do |direction|
            # TODO: I can't support a weight and a color?
            cell.change_border(direction, color || weight)
          end
        else
          modifier.borders.each do |direction|
            cell.change_border(direction, color || weight)
          end
        end
      end
      # rubocop:enable Metrics/MethodLength

      def do_fill!(cell, modifier)
        cell.change_fill(modifier.color.to_hex) if modifier.color
      end

      def do_formats!(cell, modifier)
        cell.change_font_bold(true) if modifier.formatted?('bold')
        cell.change_font_italics(true) if modifier.formatted?('italic')
        cell.change_font_underline(true) if modifier.formatted?('underline')
        cell.change_font_strikethrough(true) if modifier.formatted?('strikethrough')
      end

      def do_fonts!(cell, modifier)
        cell.change_font_color(modifier.fontcolor.to_hex) if modifier.fontcolor
        cell.change_font_name(modifier.fontfamily) if modifier.fontfamily
        cell.change_font_size(modifier.fontsize) if modifier.fontsize
      end

      def do_number_formats!(cell, modifier)
        return unless modifier.numberformat

        cell.set_number_format(number_format_code(modifier.numberformat))
        # TODO: this is annoying... we have to set the contents with the correct type of object
        cell.change_contents(cell.value)
      end

      def number_format_code(numberformat)
        ::RubyXL::NumberFormats::DEFAULT_NUMBER_FORMATS.find_by_format_id(
          # rubocop:disable Lint/ConstantResolution
          NUM_FMT_IDS[numberformat.to_sym]
          # rubocop:enable Lint/ConstantResolution
        ).format_code
      end

      def format_cell!(row_index, cell_index, modifier)
        @worksheet.sheet_data[row_index][cell_index].tap do |cell|
          do_alignments!(cell, modifier)
          do_borders!(cell, modifier)
          do_fill!(cell, modifier)
          do_fonts!(cell, modifier)
          do_formats!(cell, modifier)
          do_number_formats!(cell, modifier)
        end
      end

      def open_workbook
        if ::File.exist?(@output_filename)
          ::RubyXL::Parser.parse(@output_filename)
        else
          ::RubyXL::Workbook.new
        end
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end
