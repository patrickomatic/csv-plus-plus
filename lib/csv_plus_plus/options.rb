# typed: true
# frozen_string_literal: true

require_relative './cli_flag'
require_relative './google_options'

module CSVPlusPlus
  # The options a user can supply (via CLI flags)
  #
  # @attr backup [boolean] Create a backup of the spreadsheet before writing
  # @attr create_if_not_exists [boolean] Create the spreadsheet if it does not exist?
  # @attr key_values [Hash] Additional variables that can be supplied to the template
  # @attr offset [Array<Integer>] An [x, y] offset (array with two integers)
  # @attr output_filename [String] The file to write our compiled results to
  # @attr sheet_name [String] The name of the spreadsheet to write to
  # @attr verbose [boolean] Include extra verbose output?
  # @attr_reader google [GoogleOptions] Options that are specific to the Google Sheets writer
  class Options
    attr_accessor :backup, :create_if_not_exists, :key_values, :offset, :output_filename, :sheet_name, :verbose
    attr_reader :google

    # initialize
    def initialize
      @offset = [0, 0]
      @create_if_not_exists = false
      @key_values = {}
      @verbose = false
      @backup = false
    end

    # Set the Google Sheet ID
    #
    # @param sheet_id [String] The identifier used by Google's API to reference the sheet.  You can find it in the URL
    #   for the sheet
    #
    # @return [String]
    def google_sheet_id=(sheet_id)
      @google = ::CSVPlusPlus::GoogleOptions.new(sheet_id)
    end

    # Returns an error string or nil if there are no validation problems
    #
    # @return [String, nil]
    def validate
      return if @google || @output_filename

      'You must supply either a Google Sheet ID or an output file'
    end

    # Return a string with a verbose description of what we're doing with the options
    #
    # @return [String]
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

    # @return [String]
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
