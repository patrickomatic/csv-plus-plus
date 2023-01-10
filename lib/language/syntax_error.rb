require_relative './execution_context'

module CSVPlusPlus
  module Language
    class SyntaxError < StandardError
      def initialize(message, bad_input, execution_context, wrapped_error: nil)
        @message = message
        @bad_input = bad_input
        @execution_context = execution_context
        @wrapped_error = wrapped_error
      end

      def to_s
        row_index = @execution_context.row_index
        line_number = @execution_context.line_number
        filename = @execution_context.filename

        if @execution_context.verbose 
          $stderr.puts @wrapped_error.full_message
        end

        line_str = line_number ? (':' + line_number.to_s) : ''
        prefix = "csv++ #{filename}#{line_str}"
        postfix = "#{@message}: \"#{@bad_input}\""

        if @execution_context.cell_index
          "#{prefix}[#{row_index},#{@execution_context.cell_index}] #{postfix}"
        elsif !row_index.nil?
          "#{prefix}[#{row_index}] #{postfix}"
        else
          "#{prefix} #{postfix}"
        end
      end
    end
  end
end
