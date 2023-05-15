# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Some shared functionality that all Writers should build on
    #
    # @attr_reader position [Position] The current position - needed to resolve variables and display useful error
    #   messages
    class Writer
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::CSVPlusPlus::Runtime::Position) }
      attr_reader :position

      protected

      sig do
        params(position: ::CSVPlusPlus::Runtime::Position).void
      end
      # Open a CSV outputter to the +output_filename+ specified by the +Options+
      #
      # @param position [Position] The current position.
      def initialize(position)
        @position = position
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
