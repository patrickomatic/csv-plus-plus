# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Some shared functionality that all Writers should build on
    #
    # @attr_reader options [Options] The supplied options - some of which are relevant for our writer instance
    # @attr_reader runtime [Runtime] The current runtime - needed to resolve variables and display useful error messages
    class BaseWriter
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::CSVPlusPlus::Options) }
      attr_reader :options

      sig { returns(::CSVPlusPlus::Runtime::Runtime) }
      attr_reader :runtime

      protected

      sig { params(options: ::CSVPlusPlus::Options, runtime: ::CSVPlusPlus::Runtime::Runtime).void }
      # Open a CSV outputter to the +output_filename+ specified by the +Options+
      #
      # @param options [Options] The supplied options.
      # @param runtime [Runtime] The current runtime.
      def initialize(options, runtime)
        @options = options
        @runtime = runtime
      end

      sig { abstract.params(template: ::CSVPlusPlus::Template).void }
      # Write the given +template+.
      #
      # @param template [Template]
      def write(template); end

      sig { abstract.void }
      # Write a backup of the current spreadsheet.
      def write_backup; end
    end
  end
end
