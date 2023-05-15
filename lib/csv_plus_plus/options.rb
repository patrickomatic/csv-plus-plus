# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Options that a user can supply - either specific for compiling to a file (xlsx, csv) or Google Sheets
  module Options
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

    sig do
      params(flags: ::T::Hash[::Symbol, ::String], input_filename: ::Pathname).returns(::CSVPlusPlus::Options::Options)
    end
    # Use the given +flags+ to determine if we're dealing with either a Google Sheets or file-based
    # compilation and build an +Options+ instance accordingly.
    #
    # @param flags [Hash<Symbol, String>]
    # @param input_filename [Pathname]
    #
    # @return [Options::Options]
    def self.from_cli_flags(flags, input_filename)
      sheet_name = flags[:'sheet-name'] || input_filename.sub_ext('').to_s
      if (google_sheet_id = flags[:'google-sheet-id'])
        ::CSVPlusPlus::Options::GoogleSheetsOptions.new(sheet_name, google_sheet_id)
      elsif (output_filename = flags[:output])
        ::CSVPlusPlus::Options::FileOptions.new(sheet_name, output_filename)
      else
        raise(::CSVPlusPlus::Error::CLIError, 'You must supply either -o/--output or -g/-google-sheet-id')
      end
    end
  end
end

require_relative './options/options'

require_relative './options/file_options'
require_relative './options/google_sheets_options'
