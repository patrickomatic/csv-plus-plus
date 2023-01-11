# frozen_string_literal: true

require_relative './execution_context'

module CSVPlusPlus
  module Language
    ##
    # An error that can be thrown for various syntax errors
    class SyntaxError < StandardError
      # initialize
      def initialize(message, bad_input, execution_context, wrapped_error: nil)
        @bad_input = bad_input.to_s
        @execution_context = execution_context
        @wrapped_error = wrapped_error
        @message = message

        super(message)
      end

      # to_s
      def to_s
        @message
      end

      # Output a user-helpful string that references the relevant indexes and line number
      def to_trace
        warn(@wrapped_error.full_message) if @execution_context.verbose && @wrapped_error
        "#{message_prefix}#{cell_index} #{message_postfix}"
      end

      private

      def cell_index
        row_index = @execution_context&.row_index
        if @execution_context&.cell_index
          "[#{row_index},#{@execution_context.cell_index}]"
        elsif row_index
          "[#{row_index}]"
        else
          ''
        end
      end

      def message_prefix
        line_number = @execution_context.line_number
        filename = @execution_context.filename

        line_str = line_number ? ":#{line_number}" : ''
        "csv++ #{filename}#{line_str}"
      end

      def message_postfix
        "#{@message}: \"#{@bad_input}\""
      end
    end
  end
end
