# typed: false
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Build a RubyXL-decorated Modifier class adds some support for Excel
    class RubyXLModifier < ::SimpleDelegator
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

      # The excel-specific border weight
      #
      # @return [Integer]
      def border_weight
        return unless borderstyle

        # rubocop:disable Lint/ConstantResolution
        BORDER_STYLES[borderstyle.to_sym]
        # rubocop:enable Lint/ConstantResolution
      end

      # The excel-specific number format code
      #
      # @return [::String]
      def number_format_code
        return unless numberformat

        ::RubyXL::NumberFormats::DEFAULT_NUMBER_FORMATS.find_by_format_id(
          # rubocop:disable Lint/ConstantResolution
          NUM_FMT_IDS[numberformat.to_sym]
          # rubocop:enable Lint/ConstantResolution
        ).format_code
      end
    end
  end
end
