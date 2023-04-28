# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error thrown by our code (generally to be handled at the top level bin/ command)
    class Error < ::StandardError
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::T.nilable(::StandardError)) }
      attr_reader :wrapped_error

      sig { params(message: ::String, wrapped_error: ::T.nilable(::StandardError)).void }
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(message, wrapped_error: nil)
        super(message)

        @message = message
        @wrapped_error = wrapped_error
      end

      sig { abstract.returns(::String) }
      # Return an error message for display to a command-line user.
      #
      # @return [::String]
      def error_message; end
    end
  end
end
