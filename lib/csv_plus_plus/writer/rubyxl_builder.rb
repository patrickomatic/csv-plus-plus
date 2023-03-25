# frozen_string_literal: true

require_relative './rubyxl_modifier'

module CSVPlusPlus
  module Writer
    # Build a RubyXL workbook formatted according to the given +rows+
    #
    # @attr_reader input_filename [String] The filename being written to
    # @attr_reader rows [Array<Row>] The rows being written
    class RubyXLBuilder
      attr_reader :input_filename, :rows

      # @param input_filename [String] The file to write to
      # @param rows [Array<Row>] The rows to write
      # @param runtime [Runtime] The current runtime
      # @param sheet_name [String] The name of the sheet within the workbook to write to
      def initialize(input_filename:, rows:, runtime:, sheet_name:)
        @rows = rows
        @input_filename = input_filename
        @runtime = runtime
        @sheet_name = sheet_name
      end

      # Build a +RubyXL::Workbook+ with the given +@rows+ in +sheet_name+
      #
      # @return [RubyXL::Workbook]
      def build_workbook
        open_workbook.tap { build_workbook! }
      end

      private

      def build_workbook!
        @runtime.map_rows(@rows, cells_too: true) do |cell|
          modifier = ::CSVPlusPlus::Writer::RubyXLModifier.new(cell.modifier)

          @worksheet.add_cell(@runtime.row_index, @runtime.cell_index, cell.evaluate(@runtime))
          format_cell!(@runtime.row_index, @runtime.cell_index, modifier)
        end
      end

      def do_alignments!(cell, modifier)
        cell.change_horizontal_alignment(modifier.halign.to_s) if modifier.halign
        cell.change_vertical_alignment(modifier.valign.to_s) if modifier.valign
      end

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
        cell.change_font_bold(true) if modifier.formatted?(:bold)
        cell.change_font_italics(true) if modifier.formatted?(:italic)
        cell.change_font_underline(true) if modifier.formatted?(:underline)
        cell.change_font_strikethrough(true) if modifier.formatted?(:strikethrough)
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

      def open_workbook
        if ::File.exist?(@input_filename)
          ::RubyXL::Parser.parse(@input_filename).tap do |workbook|
            @worksheet = workbook[@sheet_name] || workbook.add_worksheet(@sheet_name)
          end
        else
          ::RubyXL::Workbook.new.tap do |workbook|
            @worksheet = workbook.worksheets[0].tap { |w| w.sheet_name = @sheet_name }
          end
        end
      end
    end
  end
end
