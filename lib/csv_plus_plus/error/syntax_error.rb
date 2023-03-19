# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that can be thrown for various syntax errors
    class SyntaxError < ::CSVPlusPlus::Error::Error
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(runtime, wrapped_error: nil)
        @runtime = runtime
        @wrapped_error = wrapped_error

        super()
      end

      # @return [String]
      def to_s
        to_trace
      end

      # Output a verbose user-helpful string that references the current runtime
      def to_verbose_trace
        warn(@wrapped_error.full_message) if @wrapped_error
        warn(@wrapped_error.backtrace) if @wrapped_error
        to_trace
      end

      # Output a user-helpful string that references the runtime state
      #
      # @return [String]
      def to_trace
        "#{message_prefix}#{cell_index} #{error_message}"
      end

      private

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

      def message_prefix
        line_number = @runtime.line_number
        filename = @runtime.filename

        line_str = line_number ? ":#{line_number}" : ''
        "#{filename}#{line_str}"
      end
    end
  end
end
