# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # Build a RubyXL-decorated Modifier class adds some support for Excel
    class RubyXLModifier < ::CSVPlusPlus::Modifier::Modifier
      extend ::T::Sig

      # @see https://www.rubydoc.info/gems/rubyXL/RubyXL/NumberFormats
      # @see https://support.microsoft.com/en-us/office/number-format-codes-5026bbd6-04bc-48cd-bf33-80f18b4eae68
      NUM_FMT_IDS = ::T.let(
        {
          ::CSVPlusPlus::Modifier::NumberFormat::Currency => 5,
          ::CSVPlusPlus::Modifier::NumberFormat::Date => 14,
          ::CSVPlusPlus::Modifier::NumberFormat::DateTime => 22,
          ::CSVPlusPlus::Modifier::NumberFormat::Number => 1,
          ::CSVPlusPlus::Modifier::NumberFormat::Percent => 9,
          ::CSVPlusPlus::Modifier::NumberFormat::Text => 49,
          ::CSVPlusPlus::Modifier::NumberFormat::Time => 21,
          ::CSVPlusPlus::Modifier::NumberFormat::Scientific => 48
        }.freeze,
        ::T::Hash[::CSVPlusPlus::Modifier::NumberFormat, ::Integer]
      )
      private_constant :NUM_FMT_IDS

      # @see http://www.datypic.com/sc/ooxml/t-ssml_ST_BorderStyle.html
      # ST_BorderStyle = %w{ none thin medium dashed dotted thick double hair mediumDashed dashDot mediumDashDot
      #                      dashDotDot slantDashDot }
      BORDER_STYLES = ::T.let(
        {
          ::CSVPlusPlus::Modifier::BorderStyle::Dashed => 'dashed',
          ::CSVPlusPlus::Modifier::BorderStyle::Dotted => 'dotted',
          ::CSVPlusPlus::Modifier::BorderStyle::Double => 'double',
          ::CSVPlusPlus::Modifier::BorderStyle::Solid => 'thin',
          ::CSVPlusPlus::Modifier::BorderStyle::SolidMedium => 'medium',
          ::CSVPlusPlus::Modifier::BorderStyle::SolidThick => 'thick'
        }.freeze,
        ::T::Hash[::CSVPlusPlus::Modifier::BorderStyle, ::String]
      )
      private_constant :BORDER_STYLES

      sig { returns(::T.nilable(::String)) }
      # The excel-specific border weight
      #
      # @return [::String, nil]
      def border_weight
        # rubocop:disable Lint/ConstantResolution
        BORDER_STYLES[borderstyle]
        # rubocop:enable Lint/ConstantResolution
      end

      sig { returns(::T.nilable(::String)) }
      # The horizontal alignment, formatted for the RubyXL API
      #
      # @return [::String, nil]
      def horizontal_alignment
        @halign&.serialize
      end

      sig { returns(::T.nilable(::String)) }
      # The excel-specific number format code
      #
      # @return [::String]
      def number_format_code
        return unless @numberformat

        ::RubyXL::NumberFormats::DEFAULT_NUMBER_FORMATS.find_by_format_id(
          # rubocop:disable Lint/ConstantResolution
          NUM_FMT_IDS[@numberformat]
          # rubocop:enable Lint/ConstantResolution
        ).format_code
      end

      sig { returns(::T.nilable(::String)) }
      # The vertical alignment, formatted for the RubyXL API
      #
      # @return [::String, nil]
      def vertical_alignment
        @valign&.serialize
      end
    end
  end
end
