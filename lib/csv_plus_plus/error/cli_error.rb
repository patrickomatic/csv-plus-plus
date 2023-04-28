# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that represents an invalid CLI option or state
    class CLIError < ::CSVPlusPlus::Error::Error
      extend ::T::Sig

      sig { override.returns(::String) }
      # @return [String]
      def error_message
        message
      end
    end
  end
end
