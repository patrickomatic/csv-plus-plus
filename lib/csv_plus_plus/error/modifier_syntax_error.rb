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

      # Create a relevant error message given +@choices+ or +@message+ (one of them must be supplied).
      #
      # @return [::String]
      def error_message
        <<~ERROR_MESSAGE
          Error parsing modifier: [[#{@wrapped_error.modifier}=...]]
          Bad input: #{@wrapped_error.bad_input}
          Reason: #{@wrapped_error.message}
        ERROR_MESSAGE
      end
    end
  end
end
