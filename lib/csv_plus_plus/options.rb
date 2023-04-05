# typed: strict
# frozen_string_literal: true

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
    extend ::T::Sig

    # The supported output formats.  We use this to dispatch flow in several places
    class OutputFormat < ::T::Enum
      enums do
        CSV = new
        Excel = new
        GoogleSheets = new
        OpenDocument = new
      end
    end

    sig { returns(::T::Boolean) }
    attr_accessor :backup

    sig { returns(::T::Boolean) }
    attr_accessor :create_if_not_exists

    sig { returns(::T::Hash[::Symbol, ::String]) }
    attr_accessor :key_values

    sig { returns(::T::Array[::Integer]) }
    attr_accessor :offset

    sig { returns(::T.nilable(::String)) }
    attr_accessor :output_filename

    sig { returns(::T.nilable(::String)) }
    attr_accessor :sheet_name

    sig { returns(::T::Boolean) }
    attr_accessor :verbose

    sig { returns(::T.nilable(::CSVPlusPlus::GoogleOptions)) }
    attr_reader :google

    sig { void }
    # Initialize a default +Options+ object
    def initialize
      @offset = ::T.let([0, 0], ::T::Array[::Integer])
      @create_if_not_exists = ::T.let(false, ::T::Boolean)
      @key_values = ::T.let({}, ::T::Hash[::Symbol, ::String])
      @verbose = ::T.let(false, ::T::Boolean)
      @backup = ::T.let(false, ::T::Boolean)
      @google = ::T.let(nil, ::T.nilable(::CSVPlusPlus::GoogleOptions))
    end

    sig { params(sheet_id: ::String).returns(::CSVPlusPlus::GoogleOptions) }
    # Set the Google Sheet ID
    #
    # @param sheet_id [::String] The identifier used by Google's API to reference the sheet.  You can find it in the URL
    #   for the sheet
    #
    # @return [::String]
    def google_sheet_id=(sheet_id)
      @google = ::CSVPlusPlus::GoogleOptions.new(sheet_id)
    end

    sig { returns(::CSVPlusPlus::Options::OutputFormat) }
    # Given the options, figure out which type of +OutputFormat+ we'll be writing to
    #
    # @return [Options::OutputFormat]
    def output_format
      return ::CSVPlusPlus::Options::OutputFormat::GoogleSheets if @google

      case @output_filename
      when /\.csv$/ then ::CSVPlusPlus::Options::OutputFormat::CSV
      when /\.ods$/ then ::CSVPlusPlus::Options::OutputFormat::OpenDocument
      when /\.xl(sx|sm|tx|tm)$/ then ::CSVPlusPlus::Options::OutputFormat::Excel
      else raise(::CSVPlusPlus::Error::Error, "Unsupported file extension: #{@output_filename}")
      end
    end

    sig { returns(::T.nilable(::String)) }
    # Returns an error string or nil if there are no validation problems
    #
    # @return [String, nil]
    def validate
      return if @google || @output_filename

      'You must supply either a Google Sheet ID or an output file'
    end

    sig { returns(::String) }
    # Return a string with a verbose description of what we're doing with the options
    #
    # @return [String]
    def verbose_summary
      <<~SUMMARY
        #{summary_divider}

        # csv++ Command Options

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

    private

    sig { returns(::String) }
    def summary_divider
      '========================================================================='
    end
  end
end
