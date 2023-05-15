# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Build a RubyXL workbook formatted according to the given +rows+
    #
    # @attr_reader output_filename [Pathname, nil] The filename being written to
    # @attr_reader rows [Array<Row>] The rows being written
    # rubocop:disable Metrics/ClassLength
    class RubyXLBuilder
      extend ::T::Sig
      include ::CSVPlusPlus::Writer::Merger

      RubyXLCell = ::T.type_alias { ::T.all(::RubyXL::Cell, ::RubyXL::CellConvenienceMethods) }
      public_constant :RubyXLCell

      RubyXLValue = ::T.type_alias { ::T.any(::String, ::Numeric, ::Date) }
      public_constant :RubyXLValue

      sig { returns(::T::Array[::CSVPlusPlus::Row]) }
      attr_reader :rows

      sig do
        params(
          options: ::CSVPlusPlus::Options::FileOptions,
          position: ::CSVPlusPlus::Runtime::Position,
          rows: ::T::Array[::CSVPlusPlus::Row]
        ).void
      end
      # @param options [Options::FileOptions]
      # @param position [Position] The current position
      # @param rows [Array<Row>] The rows to write
      def initialize(options:, position:, rows:)
        @options = options
        @position = position
        @rows = rows
        @worksheet = ::T.let(open_worksheet, ::RubyXL::Worksheet)
        @reader = ::T.let(::CSVPlusPlus::Reader::RubyXL.new(@options, @worksheet), ::CSVPlusPlus::Reader::RubyXL)
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
      def build_workbook!
        @position.map_all_cells(@rows) do |cell|
          build_cell(cell)
          format_cell!(cell)
        end
      end

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

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLValue)) }
      def value_to_rubyxl(cell)
        return cell.value unless (ast = cell.ast)

        case ast
        when ::CSVPlusPlus::Entities::Number, ::CSVPlusPlus::Entities::Date
          ast.value
        else
          ast.evaluate(@position)
        end
      end

      sig do
        params(
          cell: ::CSVPlusPlus::Cell,
          existing_value: ::T.nilable(::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLValue)
        ).returns(::T.nilable(::CSVPlusPlus::Writer::RubyXLBuilder::RubyXLValue))
      end
      def merge_value(cell, existing_value)
        merge_cell_value(existing_value:, new_value: value_to_rubyxl(cell), options: @options)
      end

      sig { params(cell: ::CSVPlusPlus::Cell).void }
      def format_cell!(cell)
        modifier = ::T.cast(cell.modifier, ::CSVPlusPlus::Modifier::RubyXLModifier)
        @reader.value_at(cell).tap do |rubyxl_cell|
          do_alignments!(rubyxl_cell, modifier)
          do_borders!(rubyxl_cell, modifier)
          do_fill!(rubyxl_cell, modifier)
          do_fonts!(rubyxl_cell, modifier)
          do_formats!(rubyxl_cell, modifier)
          do_number_formats!(rubyxl_cell, modifier)
        end
      end

      sig { returns(::RubyXL::Worksheet) }
      def open_worksheet
        if ::File.exist?(@options.output_filename)
          workbook = ::RubyXL::Parser.parse(@options.output_filename)
          workbook[@options.sheet_name] || workbook.add_worksheet(@options.sheet_name)
        else
          workbook = ::RubyXL::Workbook.new
          workbook.worksheets[0].tap { |w| w.sheet_name = @options.sheet_name }
        end
      end

      sig { params(cell: ::CSVPlusPlus::Cell).void }
      # rubocop:disable Metrics/MethodLength
      def build_cell(cell)
        if (existing_cell = @reader.value_at(cell))
          merged_value = merge_value(cell, existing_cell.value)
          existing_cell.change_contents(
            cell.ast ? '' : merged_value,
            cell.ast ? merged_value : nil
          )
        elsif (ast = cell.ast)
          @worksheet.add_cell(cell.row_index, cell.index, '', ast.evaluate(@position))
        else
          @worksheet.add_cell(cell.row_index, cell.index, cell.value)
        end
      end
      # rubocop:enable Metrics/MethodLength
    end
    # rubocop:enable Metrics/ClassLength
  end
end
