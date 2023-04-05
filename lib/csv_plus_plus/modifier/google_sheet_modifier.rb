# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # Decorate a Modifier so it can be written to the Google Sheets API
    class GoogleSheetModifier < ::CSVPlusPlus::Modifier::ValidatedModifier
      extend ::T::Sig

      sig { returns(::Google::Apis::SheetsV4::Border) }
      # Format the border for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Border]
      def border
        # TODO: allow different border styles per side?
        ::Google::Apis::SheetsV4::Border.new(color: bordercolor&.to_s || '#000000', style: borderstyle.serialize)
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Color)) }
      # Format the color for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Color]
      def color
        google_sheets_color(super) if super
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Color)) }
      # Format the fontcolor for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Color]
      def fontcolor
        google_sheets_color(super) if super
      end

      sig { returns(::String) }
      # Format the halign for Google Sheets
      #
      # @return [::String]
      def halign
        super&.serialize&.upcase
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::NumberFormat)) }
      # Format the numberformat for Google Sheets
      #
      # @return [::Google::Apis::SheetsV4::NumberFormat]
      def numberformat
        ::Google::Apis::SheetsV4::NumberFormat.new(type: super) if super
      end

      sig { returns(::Google::Apis::SheetsV4::TextFormat) }
      # Builds a SheetsV4::TextFormat with the underlying Modifier
      #
      # @return [::Google::Apis::SheetsV4::TextFormat]
      def text_format
        ::Google::Apis::SheetsV4::TextFormat.new(
          bold: formatted?(::CSVPlusPlus::Modifier::TextFormat::Bold) || nil,
          italic: formatted?(::CSVPlusPlus::Modifier::TextFormat::Italic) || nil,
          strikethrough: formatted?(::CSVPlusPlus::Modifier::TextFormat::Strikethrough) || nil,
          underline: formatted?(::CSVPlusPlus::Modifier::TextFormat::Underline) || nil,
          font_family: fontfamily,
          font_size: fontsize,
          foreground_color: fontcolor
        )
      end

      sig { returns(::String) }
      # Format the valign for Google Sheets
      def valign
        super&.serialize&.upcase
      end

      private

      sig { params(color: ::CSVPlusPlus::Color).returns(::Google::Apis::SheetsV4::Color) }
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
