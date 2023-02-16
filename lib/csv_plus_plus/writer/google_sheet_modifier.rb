# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Decorate a Modifier so it can be written to the Google Sheets API
    class GoogleSheetModifier < ::SimpleDelegator
      # Format the halign for Google Sheets
      def halign
        super&.upcase
      end

      # Format the valign for Google Sheets
      def valign
        super&.upcase
      end

      # Format the color for Google Sheets
      def color
        google_sheets_color(super) if super
      end

      # Format the fontcolor for Google Sheets
      def fontcolor
        google_sheets_color(super) if super
      end

      # Format the numberformat for Google Sheets
      def numberformat
        ::Google::Apis::SheetsV4::NumberFormat.new(type: super) if super
      end

      # Builds a SheetsV4::TextFormat with the underlying Modifier
      def text_format
        ::Google::Apis::SheetsV4::TextFormat.new(
          bold: formatted?('bold') || nil,
          italic: formatted?('italic') || nil,
          strikethrough: formatted?('strikethrough') || nil,
          underline: formatted?('underline') || nil,
          font_family: fontfamily,
          font_size: fontsize,
          foreground_color: fontcolor
        )
      end

      private

      def google_sheets_color(color)
        ::Google::Apis::SheetsV4::Color.new(
          red: color.red_percent,
          green: color.green_percent,
          blue: color.blue_percent
        )
      end
    end
  end
end
