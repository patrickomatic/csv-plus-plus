# frozen_string_literal: true

require_relative './compiler'

module CSVPlusPlus
  module Language
    ##
    # An error that can be thrown for various syntax errors
    class SyntaxError < StandardError
      # TODO: this should probably take a compiler_state rather than an entire compiler
      # initialize
      def initialize(message, bad_input, compiler, wrapped_error: nil)
        @bad_input = bad_input.to_s
        @compiler = compiler
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
        warn(@wrapped_error.full_message) if @compiler.verbose && @wrapped_error
        "#{message_prefix}#{cell_index} #{message_postfix}"
      end

      private

      def cell_index
        row_index = @compiler&.row_index
        if @compiler.cell_index
          "[#{row_index},#{@compiler.cell_index}]"
        elsif row_index
          "[#{row_index}]"
        else
          ''
        end
      end

      def message_prefix
        line_number = @compiler.line_number
        filename = @compiler.filename

        line_str = line_number ? ":#{line_number}" : ''
        "csv++ #{filename}#{line_str}"
      end

      def message_postfix
        "#{@message}: \"#{@bad_input}\""
      end
    end
  end
end
