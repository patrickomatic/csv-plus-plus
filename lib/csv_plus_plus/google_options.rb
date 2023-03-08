# frozen_string_literal: true

module CSVPlusPlus
  # The Google-specific options a user can supply
  #
  # attr sheet_id [String] The ID of the Google Sheet to write to
  GoogleOptions =
    ::Struct.new(:sheet_id) do
      # Format a string with a verbose description of what we're doing with the options
      #
      # @return [String]
      def verbose_summary
        <<~SUMMARY
          ## Google Sheets Options

          > Sheet ID | #{sheet_id}
        SUMMARY
      end

      # @return [String]
      def to_s
        "GoogleOptions(sheet_id: #{sheet_id})"
      end
    end

  public_constant :GoogleOptions
end
