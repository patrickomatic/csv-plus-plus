# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that can be thrown for various syntax errors
    class SyntaxError < ::CSVPlusPlus::Error::Error
      extend ::T::Sig

      sig { params(runtime: ::CSVPlusPlus::Runtime::Runtime, wrapped_error: ::T.nilable(::StandardError)).void }
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(runtime, wrapped_error: nil)
        @runtime = runtime
        @wrapped_error = wrapped_error

        super()
      end

      sig { returns(::String) }
      # @return [::String]
      def to_s
        to_trace
      end

      # TODO: clean up all these different string-formatting error classes
      sig { override.returns(::String) }
      # @return [::String]
      def error_message
        ''
      end

      sig { returns(::String) }
      # Output a verbose user-helpful string that references the current runtime
      def to_verbose_trace
        warn(@wrapped_error.full_message) if @wrapped_error
        warn((@wrapped_error.backtrace || []).join("\n")) if @wrapped_error&.backtrace
        to_trace
      end

      sig { returns(::String) }
      # Output a user-helpful string that references the runtime state
      #
      # @return [String]
      def to_trace
        "#{message_prefix}#{cell_index} #{error_message}"
      end

      private

      sig { returns(::String) }
      def cell_index
        row_index = @runtime.row_index
        if @runtime.cell_index
          "[#{row_index},#{@runtime.cell_index}]"
        elsif row_index
          "[#{row_index}]"
        else
          ''
        end
      end

      sig { returns(::String) }
      def message_prefix
        line_number = @runtime.line_number
        filename = @runtime.filename

        # TODO: use the runtime's dirty state here since line_number is always set
        # line_str = line_number ? ":#{line_number}" : ''
        line_str = ":#{line_number}"
        "#{filename}#{line_str}"
      end
    end
  end
end
