# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Some shared functionality that all Writers should build on
    #
    # @attr_reader options [Options] The supplied options - some of which are relevant for our writer instance
    # @attr_reader position [Position] The current position - needed to resolve variables and display useful error
    #   messages
    class BaseWriter
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::CSVPlusPlus::Options) }
      attr_reader :options

      sig { returns(::CSVPlusPlus::Runtime::Position) }
      attr_reader :position

      protected

      sig { params(options: ::CSVPlusPlus::Options, position: ::CSVPlusPlus::Runtime::Position).void }
      # Open a CSV outputter to the +output_filename+ specified by the +Options+
      #
      # @param options [Options] The supplied options.
      # @param position [Position] The current position.
      def initialize(options, position)
        @options = options
        @position = position
      end

      sig { abstract.params(template: ::CSVPlusPlus::Template).void }
      # Write the given +template+.
      #
      # @param template [Template]
      def write(template); end

      sig { abstract.params(options: ::CSVPlusPlus::Options).void }
      # Write a backup of the current spreadsheet.
      def write_backup(options); end
    end
  end
end
