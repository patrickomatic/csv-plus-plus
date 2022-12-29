module CSVPlusPlus
  class SyntaxError < StandardError
    def initialize(message, input, 
                   row_number: nil, cell_number: nil, line_number: nil)
      @message = message
      @input = input
      @line_number = line_number
      @row_number = row_number
      @cell_number = cell_number
    end

    # XXX include the filename in here too
    def to_s
      if @row_number
        "csv++: #@message #@row_number:#@cell_number: #@input"
      elsif @line_number
        "csv++: #@message #@line_number: #@input"
      else
        "csv++: #@message: #@input"
      end
    end
  end
end
