# frozen_string_literal: true

require_relative './syntax_error'

module CSVPlusPlus
  module Error
    # An Error that wraps a +ModifierValidationError+ with a +Runtime+.
    class ModifierSyntaxError < ::CSVPlusPlus::Error::SyntaxError
      # You must supply either a +choices+ or +message+
      #
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [ModifierValidationError] The validtion error that this is wrapping
      def initialize(runtime, wrapped_error:)
        @wrapped_error = wrapped_error

        super(runtime, wrapped_error:)
      end

      # Calls +wrapped_error.error_message+.
      #
      # @return [::String]
      def error_message
        @wrapped_error.error_message
      end
    end
  end
end
