# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # The Google-specific options a user can supply
  #
  # attr sheet_id [String] The ID of the Google Sheet to write to
  class GoogleOptions
    extend ::T::Sig

    sig { params(sheet_id: ::String).void }
    # @param sheet_id [String] The unique ID Google uses to reference the sheet
    def initialize(sheet_id)
      @sheet_id = sheet_id
    end

    sig { returns(::String) }
    # Format a string with a verbose description of what we're doing with the options
    #
    # @return [String]
    def verbose_summary
      <<~SUMMARY
        ## Google Sheets Options

        > Sheet ID | #{@sheet_id}
      SUMMARY
    end
  end
end
