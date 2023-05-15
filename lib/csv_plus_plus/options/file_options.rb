# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Options
    # The options that are specific for compiling to a file
    #
    # @attr output_filename [Pathname] The file to write our compiled results to
    class FileOptions < ::CSVPlusPlus::Options::Options
      extend ::T::Sig
      extend ::T::Helpers

      sig { returns(::Pathname) }
      attr_accessor :output_filename

      sig { params(sheet_name: ::String, output_filename: ::String).void }
      # Initialize an +Options+ object for writing to a file
      def initialize(sheet_name, output_filename)
        super(sheet_name)

        @output_filename = ::T.let(::Pathname.new(output_filename), ::Pathname)
      end

      sig { override.returns(::CSVPlusPlus::Options::OutputFormat) }
      # Given the options, figure out which type of +OutputFormat+ we'll be writing to
      #
      # @return [Options::OutputFormat]
      def output_format
        case output_filename.extname
        when '.csv' then ::CSVPlusPlus::Options::OutputFormat::CSV
        when '.ods' then ::CSVPlusPlus::Options::OutputFormat::OpenDocument
        when /\.xl(sx|sm|tx|tm)$/ then ::CSVPlusPlus::Options::OutputFormat::Excel
        else raise(::CSVPlusPlus::Error::CLIError, "Unsupported file extension: #{output_filename}")
        end
      end

      sig { override.returns(::String) }
      # Verbose summary for options specific to compiling to a file
      #
      # @return [String]
      def verbose_summary
        shared_summary(
          <<~OUTPUT)
            > Output filename                     | #{output_filename}
          OUTPUT
      end
    end
  end
end
