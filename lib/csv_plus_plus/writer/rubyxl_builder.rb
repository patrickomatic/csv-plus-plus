# frozen_string_literal: true

require_relative './rubyxl_modifier'

module CSVPlusPlus
  module Writer
    # Build a RubyXL workbook formatted according to the given +rows+
    class RubyXLBuilder
      attr_reader :output_filename, :rows

      # initialize
      def initialize(output_filename:, rows:, sheet_name:)
        @rows = rows
        @output_filename = output_filename
        @workbook = open_workbook(sheet_name)
        @worksheet = @workbook[sheet_name]
      end

      # write the given @rows in +sheet_name+ to +@output_filename+
      def write
        build_workbook!
        @workbook.write(@output_filename)
      end

      private

      def build_workbook!
        @rows.each_with_index do |row, x|
          row.cells.each_with_index do |cell, y|
            modifier = ::CSVPlusPlus::Writer::RubyXLModifier.new(cell.modifier)

            @worksheet.add_cell(x, y, cell.to_csv)
            format_cell!(x, y, modifier)
          end
        end
      end

      # rubocop:disable Metrics/MethodLength, Metrics/PerceivedComplexity
      def do_alignments!(cell, modifier)
        # TODO: make the main modifier work this way
        # cell.change_horizontal_alignment(modifier.horizontal_alignment) if modifier.has_horizontal_alignment?
        # cell.change_vertical_alignment(modifier.vertical_alignment) if modifier.has_vertical_alignment?
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

      # rubocop:disable Metrics/MethodLength
      def do_borders!(cell, modifier)
        return unless modifier.any_border?

        color = modifier.bordercolor
        weight = modifier.border_weight

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

        cell.set_number_format(modifier.number_format_code)
        # TODO: this is annoying... we have to set the contents with the correct type of object
        cell.change_contents(cell.value)
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

      def open_workbook(sheet_name)
        if ::File.exist?(@output_filename)
          ::RubyXL::Parser.parse(@output_filename).tap do |workbook|
            workbook.add_worksheet(sheet_name) unless workbook[sheet_name]
          end
        else
          ::RubyXL::Workbook.new.tap do |workbook|
            workbook.worksheets[0].sheet_name = sheet_name
          end
        end
      end
    end
  end
end
