# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Build a RubyXL workbook formatted according to the given +rows+
    #
    # @attr_reader input_filename [String] The filename being written to
    # @attr_reader rows [Array<Row>] The rows being written
    # rubocop:disable Metrics/ClassLength
    class RubyXLBuilder
      extend ::T::Sig

      RubyXLCell = ::T.type_alias { ::T.all(::RubyXL::Cell, ::RubyXL::CellConvenienceMethods) }
      public_constant :RubyXLCell

      sig { returns(::T.nilable(::String)) }
      attr_reader :input_filename

      sig { returns(::T::Array[::CSVPlusPlus::Row]) }
      attr_reader :rows

      sig do
        params(
          input_filename: ::T.nilable(::String),
          position: ::CSVPlusPlus::Runtime::Position,
          rows: ::T::Array[::CSVPlusPlus::Row],
          sheet_name: ::T.nilable(::String)
        ).void
      end
      # @param input_filename [::String] The file to write to
      # @param position [Position] The current position
      # @param rows [Array<Row>] The rows to write
      # @param sheet_name [::String] The name of the sheet within the workbook to write to
      def initialize(input_filename:, position:, rows:, sheet_name: nil)
        @rows = rows
        @input_filename = input_filename
        @position = position
        @sheet_name = sheet_name
        @worksheet = ::T.let(open_worksheet, ::RubyXL::Worksheet)
      end

      sig { returns(::RubyXL::Workbook) }
      # Build a +RubyXL::Workbook+ with the given +@rows+ in +sheet_name+
      #
      # @return [RubyXL::Workbook]
      def build_workbook
        build_workbook!
        @worksheet.workbook
      end

      private

      sig { void }
      # rubocop:disable Metrics/MethodLength
      def build_workbook!
        @position.map_all_cells(@rows) do |cell|
          value = cell.evaluate(@position)
          if value&.start_with?('=')
            @worksheet.add_cell(@position.row_index, @position.cell_index, '', value.gsub(/^=/, ''))
          else
            @worksheet.add_cell(@position.row_index, @position.cell_index, value)
          end

          format_cell!(
            @position.row_index,
            @position.cell_index,
            ::T.cast(cell.modifier, ::CSVPlusPlus::Modifier::RubyXLModifier)
          )
        end
      end
      # rubocop:enable Metrics/MethodLength

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
      end
      def do_alignments!(cell, modifier)
        cell.change_horizontal_alignment(modifier.horizontal_alignment) if modifier.halign
        cell.change_vertical_alignment(modifier.vertical_alignment) if modifier.valign
      end

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
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
            # TODO: move direction.serialize into the RubyXLModifier
            cell.change_border(direction.serialize, color || weight)
          end
        end
      end
      # rubocop:enable Metrics/MethodLength

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
      end
      def do_fill!(cell, modifier)
        return unless modifier.color

        cell.change_fill(modifier.color&.to_hex)
      end

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
      end
      def do_formats!(cell, modifier)
        cell.change_font_bold(true) if modifier.formatted?(::CSVPlusPlus::Modifier::TextFormat::Bold)
        cell.change_font_italics(true) if modifier.formatted?(::CSVPlusPlus::Modifier::TextFormat::Italic)
        cell.change_font_underline(true) if modifier.formatted?(::CSVPlusPlus::Modifier::TextFormat::Underline)
        cell.change_font_strikethrough(true) if modifier.formatted?(::CSVPlusPlus::Modifier::TextFormat::Strikethrough)
      end

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
      end
      def do_fonts!(cell, modifier)
        cell.change_font_color(::T.must(modifier.fontcolor).to_hex) if modifier.fontcolor
        cell.change_font_name(modifier.fontfamily) if modifier.fontfamily
        cell.change_font_size(modifier.fontsize) if modifier.fontsize
      end

      sig do
        params(
          cell: ::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLCell,
          modifier: ::CSVPlusPlus::Modifier::RubyXLModifier
        ).void
      end
      def do_number_formats!(cell, modifier)
        return unless modifier.numberformat

        cell.set_number_format(modifier.number_format_code)
        # TODO: this is annoying... we have to set the contents with the correct type of object
        cell.change_contents(cell.value)
      end

      sig do
        params(row_index: ::Integer, cell_index: ::Integer, modifier: ::CSVPlusPlus::Modifier::RubyXLModifier).void
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

      sig { returns(::RubyXL::Worksheet) }
      def open_worksheet
        if @input_filename && ::File.exist?(@input_filename)
          workbook = ::RubyXL::Parser.parse(@input_filename)
          workbook[@sheet_name] || workbook.add_worksheet(@sheet_name)
        else
          workbook = ::RubyXL::Workbook.new
          workbook.worksheets[0].tap { |w| w.sheet_name = @sheet_name }
        end
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end
