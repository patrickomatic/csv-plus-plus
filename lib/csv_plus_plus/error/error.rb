# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error thrown by our code (generally to be handled at the top level bin/ command)
    class Error < StandardError
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { abstract.returns(::String) }
      # Return an error message for display to a command-line user.
      #
      # @return [::String]
      def error_message; end
    end
  end
end
