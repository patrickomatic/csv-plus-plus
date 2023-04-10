# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # Decorate a +Modifier+ so it is more compatible with the Google Sheets API
    class GoogleSheetModifier < ::CSVPlusPlus::Modifier::Modifier
      extend ::T::Sig

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Color)) }
      # Format the color for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Color]
      def background_color
        google_sheets_color(@color) if @color
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Border)) }
      # Format the border for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Border]
      def border
        return unless any_border?

        # TODO: allow different border styles per side?
        ::Google::Apis::SheetsV4::Border.new(
          color: google_sheets_color(bordercolor || ::CSVPlusPlus::Color.new('#000000')),
          style: border_style
        )
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Color)) }
      # Format the fontcolor for Google Sheets
      #
      # @return [Google::Apis::SheetsV4::Color]
      def font_color
        google_sheets_color(@fontcolor) if @fontcolor
      end

      sig { returns(::T.nilable(::String)) }
      # Format the halign for Google Sheets
      #
      # @return [::String]
      def horizontal_alignment
        halign&.serialize&.upcase
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::NumberFormat)) }
      # Format the numberformat for Google Sheets
      #
      #
      # @return [Google::Apis::SheetsV4::NumberFormat]
      def number_format
        ::Google::Apis::SheetsV4::NumberFormat.new(type: number_format_type(@numberformat)) if @numberformat
      end

      sig { returns(::Google::Apis::SheetsV4::TextFormat) }
      # Builds a SheetsV4::TextFormat with the underlying Modifier
      #
      # @return [Google::Apis::SheetsV4::TextFormat]
      def text_format
        ::Google::Apis::SheetsV4::TextFormat.new(
          bold: formatted?(::CSVPlusPlus::Modifier::TextFormat::Bold) || nil,
          italic: formatted?(::CSVPlusPlus::Modifier::TextFormat::Italic) || nil,
          strikethrough: formatted?(::CSVPlusPlus::Modifier::TextFormat::Strikethrough) || nil,
          underline: formatted?(::CSVPlusPlus::Modifier::TextFormat::Underline) || nil,
          font_family: fontfamily,
          font_size: fontsize,
          foreground_color: font_color
        )
      end

      sig { returns(::T.nilable(::String)) }
      # Format the valign for Google Sheets
      def vertical_alignment
        valign&.serialize&.upcase
      end

      private

      sig { returns(::T.nilable(::String)) }
      # Format the border style for Google Sheets
      #
      # @see https://developers.google.com/apps-script/reference/spreadsheet/border-style
      #
      # @return [::String, nil]
      # rubocop:disable Metrics/CyclomaticComplexity
      def border_style
        return 'SOLID' unless @borderstyle

        case @borderstyle
        when ::CSVPlusPlus::Modifier::BorderStyle::Dashed then 'DASHED'
        when ::CSVPlusPlus::Modifier::BorderStyle::Dotted then 'DOTTED'
        when ::CSVPlusPlus::Modifier::BorderStyle::Double then 'DOUBLE'
        when ::CSVPlusPlus::Modifier::BorderStyle::Solid then 'SOLID'
        when ::CSVPlusPlus::Modifier::BorderStyle::SolidMedium then 'SOLID_MEDIUM'
        when ::CSVPlusPlus::Modifier::BorderStyle::SolidThick then 'SOLID_THICK'
        else ::T.absurd(@borderstyle)
        end
      end
      # rubocop:enable Metrics/CyclomaticComplexity

      sig { params(color: ::CSVPlusPlus::Color).returns(::Google::Apis::SheetsV4::Color) }
      def google_sheets_color(color)
        ::Google::Apis::SheetsV4::Color.new(
          red: color.red_percent,
          green: color.green_percent,
          blue: color.blue_percent
        )
      end

      sig { params(numberformat: ::CSVPlusPlus::Modifier::NumberFormat).returns(::String) }
      # @see https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/cells#NumberFormat
      #
      # @return [::String]
      # rubocop:disable Metrics/CyclomaticComplexity, Metrics/MethodLength
      def number_format_type(numberformat)
        case numberformat
        when ::CSVPlusPlus::Modifier::NumberFormat::Currency then 'CURRENCY'
        when ::CSVPlusPlus::Modifier::NumberFormat::Date then 'DATE'
        when ::CSVPlusPlus::Modifier::NumberFormat::DateTime then 'DATE_TIME'
        when ::CSVPlusPlus::Modifier::NumberFormat::Number then 'NUMBER'
        when ::CSVPlusPlus::Modifier::NumberFormat::Percent then 'PERCENT'
        when ::CSVPlusPlus::Modifier::NumberFormat::Text then 'TEXT'
        when ::CSVPlusPlus::Modifier::NumberFormat::Time then 'TIME'
        when ::CSVPlusPlus::Modifier::NumberFormat::Scientific then 'SCIENTIFIC'
        else ::T.absurd(numberformat)
        end
      end
      # rubocop:enable Metrics/CyclomaticComplexity, Metrics/MethodLength
    end
  end
end
