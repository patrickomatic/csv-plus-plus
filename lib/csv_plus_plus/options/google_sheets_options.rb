# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Options
    # The Google-specific options a user can supply.
    #
    # @attr sheet_id [String] The ID of the Google Sheet to write to.
    class GoogleSheetsOptions < Options
      extend ::T::Sig

      sig { returns(::String) }
      attr_reader :sheet_id

      sig { params(sheet_name: ::String, sheet_id: ::String).void }
      # @param sheet_name [String] The name of the sheet
      # @param sheet_id [String] The unique ID Google uses to reference the sheet
      def initialize(sheet_name, sheet_id)
        super(sheet_name)

        @sheet_id = sheet_id
      end

      sig { override.returns(::CSVPlusPlus::Options::OutputFormat) }
      # @return [OutputFormat]
      def output_format
        ::CSVPlusPlus::Options::OutputFormat::GoogleSheets
      end

      sig { override.returns(::String) }
      # Format a string with a verbose description of Google-specific options
      #
      # @return [String]
      def verbose_summary
        shared_summary(
          <<~SUMMARY)
            > Sheet ID | #{@sheet_id}
          SUMMARY
      end
    end
  end
end
