# typed: strict
# frozen_string_literal: true

require_relative './writer/merger'
require_relative './writer/writer'

require_relative './writer/csv'
require_relative './writer/excel'
require_relative './writer/google_sheets'
require_relative './writer/google_sheets_builder'
require_relative './writer/open_document'
require_relative './writer/rubyxl_builder'

module CSVPlusPlus
  # Various strategies for writing to various formats (excel, google sheets, CSV & OpenDocument (not yet implemented))
  module Writer
    extend ::T::Sig

    sig do
      params(
        options: ::CSVPlusPlus::Options::Options,
        position: ::CSVPlusPlus::Runtime::Position
      ).returns(
        ::T.any(
          ::CSVPlusPlus::Writer::CSV,
          ::CSVPlusPlus::Writer::Excel,
          ::CSVPlusPlus::Writer::GoogleSheets,
          ::CSVPlusPlus::Writer::OpenDocument
        )
      )
    end
    # Return an instance of a writer depending on the given +options+
    #
    # @param options [Options] The supplied options.
    # @param position [Position] The current position.
    #
    # @return [Writer::CSV | Writer::Excel | Writer::GoogleSheets | Writer::OpenDocument]
    def self.writer(options, position)
      output_format = options.output_format
      case output_format
      when ::CSVPlusPlus::Options::OutputFormat::CSV then csv(options, position)
      when ::CSVPlusPlus::Options::OutputFormat::Excel then excel(options, position)
      when ::CSVPlusPlus::Options::OutputFormat::GoogleSheets then google_sheets(options, position)
      when ::CSVPlusPlus::Options::OutputFormat::OpenDocument then open_document(options, position)
      else ::T.absurd(output_format)
      end
    end

    sig do
      params(
        options: ::CSVPlusPlus::Options::Options,
        position: ::CSVPlusPlus::Runtime::Position
      ).returns(::CSVPlusPlus::Writer::CSV)
    end
    # Instantiate a +CSV+ writer
    def self.csv(options, position)
      ::CSVPlusPlus::Writer::CSV.new(::T.cast(options, ::CSVPlusPlus::Options::FileOptions), position)
    end

    sig do
      params(
        options: ::CSVPlusPlus::Options::Options,
        position: ::CSVPlusPlus::Runtime::Position
      ).returns(::CSVPlusPlus::Writer::Excel)
    end
    # Instantiate a +Excel+ writer
    def self.excel(options, position)
      ::CSVPlusPlus::Writer::Excel.new(::T.cast(options, ::CSVPlusPlus::Options::FileOptions), position)
    end

    sig do
      params(
        options: ::CSVPlusPlus::Options::Options,
        position: ::CSVPlusPlus::Runtime::Position
      ).returns(::CSVPlusPlus::Writer::GoogleSheets)
    end
    # Instantiate a +GoogleSheets+ writer
    def self.google_sheets(options, position)
      ::CSVPlusPlus::Writer::GoogleSheets.new(::T.cast(options, ::CSVPlusPlus::Options::GoogleSheetsOptions), position)
    end

    sig do
      params(
        options: ::CSVPlusPlus::Options::Options,
        position: ::CSVPlusPlus::Runtime::Position
      ).returns(::CSVPlusPlus::Writer::OpenDocument)
    end
    # Instantiate an +OpenDocument+ writer
    def self.open_document(options, position)
      ::CSVPlusPlus::Writer::OpenDocument.new(::T.cast(options, ::CSVPlusPlus::Options::FileOptions), position)
    end
  end
end
