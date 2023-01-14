# frozen_string_literal: true

module CSVPlusPlus
  module Language
    ##
    # An error that can be thrown for various syntax errors
    class SyntaxError < StandardError
      # initialize
      def initialize(message, bad_input, runtime, wrapped_error: nil)
        @bad_input = bad_input.to_s
        @runtime = runtime
        @wrapped_error = wrapped_error
        @message = message

        super(message)
      end

      # to_s
      def to_s
        to_trace
      end

      # Output a verbose user-helpful string that references the current runtime
      def to_verbose_trace
        warn(@wrapped_error.full_message)
        to_trace
      end

      # Output a user-helpful string that references the runtime state
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
