# frozen_string_literal: true

require_relative './google_options'

module CSVPlusPlus
  # The options a user can supply
  class Options
    attr_accessor :backup, :create_if_not_exists, :key_values, :offset, :output_filename, :sheet_name, :verbose
    attr_reader :google

    # initialize
    def initialize
      @offset = [0, 0]
      @create_if_not_exists = false
      @key_values = {}
      @verbose = false
      # TODO: switch to true? probably a safer choice
      @backup = false
    end

    # Set the Google Sheet ID
    def google_sheet_id=(sheet_id)
      @google = ::CSVPlusPlus::GoogleOptions.new(sheet_id)
    end

    # Returns an error string or nil if there are no validation problems
    def validate
      return if @google || @output_filename

      'You must supply either a Google Sheet ID or an output file'
    end

    # Return a string with a verbose description of what we're doing with the options
    def verbose_summary
      <<~SUMMARY
        #{summary_divider}

        # csv++ Command Options

        > Input filename                      | #{@filename}
        > Sheet name                          | #{@sheet_name}
        > Create sheet if it does not exist?  | #{@create_if_not_exists}
        > Spreadsheet row-offset              | #{@offset[0]}
        > Spreadsheet cell-offset             | #{@offset[1]}
        > User-supplied key-values            | #{@key_values}
        > Verbose                             | #{@verbose}

        ## Output Options

        > Backup                              | #{@backup}
        > Output filename                     | #{@output_filename}

        #{@google&.verbose_summary || ''}
        #{summary_divider}
      SUMMARY
    end

    # to_s
    def to_s
      "Options(create_if_not_exists: #{@create_if_not_exists}, google: #{@google}, key_values: #{@key_values}, " \
        "offset: #{@offset}, sheet_name: #{@sheet_name}, verbose: #{@verbose})"
    end

    private

    def summary_divider
      '========================================================================='
    end
  end
end
