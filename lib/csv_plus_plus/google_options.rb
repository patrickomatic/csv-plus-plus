# frozen_string_literal: true

module CSVPlusPlus
  # The Google-specific options a user can supply
  GoogleOptions =
    ::Struct.new(:sheet_id) do
      # Return a string with a verbose description of what we're doing with the options
      def verbose_summary
        <<~SUMMARY
          ## Google Sheets Options

          > Sheet ID | #{sheet_id}
        SUMMARY
      end

      # to_s
      def to_s
        "GoogleOptions(sheet_id: #{sheet_id})"
      end
    end

  public_constant :GoogleOptions
end
