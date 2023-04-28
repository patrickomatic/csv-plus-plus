# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Individual CLI flags that a user can supply
  #
  # @attr_reader short_flag [String] A definition of the short/single-character flag
  # @attr_reader long_flag [String] A definition of the long/word-based flag
  # @attr_reader description [String] A description of what the flag does
  # @attr_reader handler [Proc(Options, String)] A proc which is called to handle when this flag is seen
  class CLIFlag
    extend ::T::Sig

    # public_constant :SUPPORTED_CSVPP_FLAGS

    sig { returns(::String) }
    attr_reader :description

    sig { returns(::T.proc.params(options: ::CSVPlusPlus::Options, v: ::String).void) }
    attr_reader :handler

    sig { returns(::String) }
    attr_reader :long_flag

    sig { returns(::String) }
    attr_reader :short_flag

    sig do
      params(
        short_flag: ::String,
        long_flag: ::String,
        description: ::String,
        handler: ::T.proc.params(options: ::CSVPlusPlus::Options, v: ::String).void
      ).void
    end
    # @param short_flag [String] A definition of the short/single-character flag
    # @param long_flag [String] A definition of the long/word-based flag
    # @param description [String] A description of what the flag does
    # @param handler [Proc(Options, String)] A proc which is called to handle when this flag is seen
    def initialize(short_flag, long_flag, description, handler)
      @short_flag = short_flag
      @long_flag = long_flag
      @description = description
      @handler = handler
    end
  end

  SUPPORTED_CSVPP_FLAGS = ::T.let(
    [
      ::CSVPlusPlus::CLIFlag.new(
        '-b',
        '--backup',
        'Create a backup of the spreadsheet before applying changes.',
        ->(options, _v) { options.backup = true }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-c',
        '--create',
        "Create the sheet if it doesn't exist.  It will use --sheet-name if specified",
        ->(options, _v) { options.create_if_not_exists = true }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-g SHEET_ID',
        '--google-sheet-id SHEET_ID',
        'The id of the sheet - you can extract this from the URL: ' \
        'https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=0',
        ->(options, v) { options.google_sheet_id = v }
      ),
      ::CSVPlusPlus::CLIFlag.new(
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
      ::CSVPlusPlus::CLIFlag.new(
        '-n SHEET_NAME',
        '--sheet-name SHEET_NAME',
        'The name of the sheet to apply the template to',
        ->(options, v) { options.sheet_name = v }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-o OUTPUT_FILE',
        '--output OUTPUT_FILE',
        'The file to write to (must be .csv, .ods, .xls)',
        ->(options, v) { options.output_filename = ::Pathname.new(v) }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-v',
        '--verbose',
        'Enable verbose output',
        lambda { |options, _v|
          options.verbose = true
        }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-x OFFSET',
        '--offset-columns OFFSET',
        'Apply the template offset by OFFSET cells',
        ->(options, v) { options.offset[0] = v }
      ),
      ::CSVPlusPlus::CLIFlag.new(
        '-y OFFSET',
        '--offset-rows OFFSET',
        'Apply the template offset by OFFSET rows',
        ->(options, v) { options.offset[1] = v }
      )
    ].freeze,
    ::T::Array[::CSVPlusPlus::CLIFlag]
  )
  public_constant :SUPPORTED_CSVPP_FLAGS
end
