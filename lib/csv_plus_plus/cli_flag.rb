# frozen_string_literal: true

require_relative './google_options'

module CSVPlusPlus
  # Individual CLI flags that a user can supply
  class CliFlag
    attr_reader :short_flag, :long_flag, :description, :handler

    # initialize
    def initialize(short_flag, long_flag, description, handler)
      @short_flag = short_flag
      @long_flag = long_flag
      @description = description
      @handler = handler
    end

    # to_s
    def to_s
      "#{@short_flag}, #{@long_flag}  #{@description}"
    end
  end
end

SUPPORTED_CSVPP_FLAGS = [
  ::CSVPlusPlus::CliFlag.new(
    '-b',
    '--backup',
    'Create a backup of the spreadsheet before applying changes.',
    ->(options, _v) { options.backup = true }
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-c',
    '--create',
    "Create the sheet if it doesn't exist.  It will use --sheet-name if specified",
    ->(options, _v) { options.create_if_not_exists = true }
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-g SHEET_ID',
    '--google-sheet-id SHEET_ID',
    'The id of the sheet - you can extract this from the URL: ' \
    'https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=0',
    ->(options, v) { options.google_sheet_id = v }
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-k',
    '--key-values KEY_VALUES',
    'A comma-separated list of key=values which will be made available to the template',
    lambda do |options, v|
      options.key_values =
        begin
          [v.split('=')].to_h
        rescue ::StandardError
          {}
        end
    end
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-n SHEET_NAME',
    '--sheet-name SHEET_NAME',
    'The name of the sheet to apply the template to',
    ->(options, v) { options.sheet_name = v }
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-o OUTPUT_FILE',
    '--output OUTPUT_FILE',
    'The file to write to (must be .csv, .ods, .xls)',
    ->(options, v) { options.output_filename = v }
  ),
  ::CSVPlusPlus::CliFlag.new('-v', '--verbose', 'Enable verbose output', ->(options, _v) { options.verbose = true }),
  ::CSVPlusPlus::CliFlag.new(
    '-x OFFSET',
    '--offset-columns OFFSET',
    'Apply the template offset by OFFSET cells',
    ->(options, v) { options.offset[0] = v }
  ),
  ::CSVPlusPlus::CliFlag.new(
    '-y OFFSET',
    '--offset-rows OFFSET',
    'Apply the template offset by OFFSET rows',
    ->(options, v) { options.offset[1] = v }
  )
].freeze
