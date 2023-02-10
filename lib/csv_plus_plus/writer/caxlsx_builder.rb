# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Build a Caxlsx object
    class CaxlsxBuilder
      # https://github.com/randym/axlsx/blob/master/lib/axlsx/stylesheet/num_fmt.rb
      # https://support.microsoft.com/en-us/office/number-format-codes-5026bbd6-04bc-48cd-bf33-80f18b4eae68
      NUM_FMTS = {
        currency: 5,
        date: 14,
        date_time: 22,
        number: 1,
        percent: 9,
        text: 49,
        time: 21,
        scientific: 48
      }.freeze
      private_constant :NUM_FMTS

      BORDER_STYLES = {
        dashed: :dashed,
        dotted: :dotted,
        double: :double,
        solid: :thin,
        solid_medium: :medium,
        solid_thick: :thick
      }.freeze
      private_constant :BORDER_STYLES

      # initialize
      def initialize(rows:)
        @rows = rows
      end

      # write the given @rows to output
      def write(sheet_name, output_filename)
        ::Axlsx::Package.new do |p|
          # TODO: how do I get the worksheet by name (I guess I need a separate lib for reading)
          p.workbook.add_worksheet(name: sheet_name) do |sheet|
            @rows.each do |row|
              build_row(p.workbook, sheet, row)
            end
          end

          p.serialize(output_filename)
        end
      end

      private

      def build_row(workbook, sheet, row)
        sheet.add_row(axlsx_row(row), style: row.cells.map { |c| build_style(workbook, c.modifier) })
      end

      def build_style(workbook, modifier)
        workbook.styles.add_style(
          alignment: build_alignment(modifier),
          bg_color: build_bg_color(modifier),
          border: build_border(modifier),
          fg_color: build_fg_color(modifier),
          num_fmt: build_num_fmt(modifier),
          sz: build_font_size(modifier),
          b: modifier.formatted?('bold'),
          u: modifier.formatted?('underline'),
          i: modifier.formatted?('italic'),
          strike: modifier.formatted?('strikethrough')
        )
      end

      def build_border(modifier)
        return unless modifier.any_border?

        {
          # rubocop:disable Lint/ConstantResolution
          style: BORDER_STYLES[modifier.borderstyle] || :medium,
          # rubocop:enable Lint/ConstantResolution
          edges: modifier.borders.map(&:downcase).map(&:to_sym),
          color: modifier.bordercolor&.to_hex || '000000'
        }
      end

      def build_alignment(modifier)
        return unless modifier.any_alignment?

        { build_horizontal_align(modifier) => build_vertical_align(modifier) }
      end

      def build_horizontal_align(modifier)
        if modifier.aligned?('left')
          :left
        elsif modifier.aligned?('right')
          :right
        else
          :center
        end
      end

      def build_vertical_align(modifier)
        if modifier.aligned?('top')
          :top
        elsif modifier.aligned?('bottom')
          :bottom
        else
          :center
        end
      end

      def build_bg_color(modifier)
        modifier.color&.to_hex
      end

      def build_font_size(modifier)
        # TODO: don't specify a default... just don't set it
        modifier.fontsize&.to_i || 14
      end

      def build_fg_color(modifier)
        modifier.fontcolor&.to_hex
      end

      def build_num_fmt(modifier)
        return unless modifier.numberformat

        # rubocop:disable Lint/ConstantResolution
        NUM_FMTS[modifier.numberformat.to_sym]
        # rubocop:enable Lint/ConstantResolution
      end

      def axlsx_row(row)
        row.cells.map(&:to_csv)
      end
    end
  end
end
