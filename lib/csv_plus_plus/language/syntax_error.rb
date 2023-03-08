# frozen_string_literal: true

module CSVPlusPlus
  module Language
    # An error that can be thrown for various syntax errors
    class SyntaxError < ::CSVPlusPlus::Error
      # @param message [String] The primary message to be shown to the user
      # @param bad_input [String] The offending input that caused the error to be thrown
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(message, bad_input, runtime, wrapped_error: nil)
        @bad_input = bad_input.to_s
        @runtime = runtime
        @wrapped_error = wrapped_error
        @message = message

        super(message)
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
        "#{message_prefix}#{cell_index} #{message_postfix}"
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
        "csv++ #{filename}#{line_str}"
      end

      def message_postfix
        "#{@message}: \"#{@bad_input}\""
      end
    end
  end
end
