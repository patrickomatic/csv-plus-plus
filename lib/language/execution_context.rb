require 'tempfile'
require_relative 'entities'
require_relative 'global_scope'

module CSVPlusPlus
  module Language
    class ExecutionContext
      attr_reader :filename, :global_scope
      attr_accessor :verbose, :cell, :cell_index, :row_index, :line_number
        
      def self.with_execution_context(input:, filename:, verbose: false)
        ec = ExecutionContext.new(input:, filename:, verbose:)
        yield ec
      rescue
        ec.unlink!
      end

      def initialize(input:, filename:, verbose:)
        @filename = filename || 'stdin'
        @verbose = verbose
        @length_of_code_section = nil
        @length_of_original_file = input.count("\n")

        # we're gonna take our input file, write it to a tmp file then each
        # step is gonna mutate that tmp file
        @tmp = Tempfile.new
        write_to_tmp!(input)
      end

      def unlink!
        if @tmp
          @tmp.close
          @tmp.unlink
          @tmp = nil
        end
      end

      def map_lines(lines, &block)
        lines.map do |line|
          block.call(line).tap do
            @row_index += 1 unless @row_index.nil?
            @line_number += 1
          end
        end
      end

      def map_row(row, &block)
        row.map.with_index do |cell, index|
          @cell = cell
          @cell_index = index
          block.call(@cell, index)
        end
      end

      def map_rows(rows, cells_too: false, &block)
        rows.map do |row|
          if cells_too
            # it's either CSV or a Row object
            cells = row&.cells || row
            ret = map_row(cells, &block)
          else
            ret = block.call(row)
          end

          @row_index += 1
          @line_number += 1

          ret
        end
      end

      def parsing_code_section!(&block)
        workflow(log_subject: "parsing code section", 
                 processing_code_section: true) do
          csv_section = block.call(@tmp)
          write_to_tmp!(csv_section)

          @length_of_csv_section = csv_section.count "\n"
          @length_of_code_section = @length_of_original_file - @length_of_csv_section
        end
      end

      def parsing_csv!(&block)
        workflow(log_subject: "parsing CSV section") do
          block.call(@tmp)
        end

        # we're done with the file and everything is in memory
        unlink!
      end

      def expanding!(&block)
        workflow(log_subject: "expanding rows") do
          block.call
        end
      end

      def resolve_static_variables!(code_section)
        # TODO this indirection seems kinda unnecessary
        @global_scope = GlobalScope.new(code_section)
        @global_scope.resolve_static_variables(code_section.variables, self)
      end

      def resolve_all_cells!(code_section, rows)
        unless @global_scope
          resolve_static_variables!(code_section)
        end

        workflow(log_subject: "resolving all cell value variable references") do
          map_rows(rows, cells_too: true) do |cell|
            if cell.ast
              cell.ast = @global_scope.resolve_cell_value(self)
            end
          end
        end
      end

      def applying_functions!(&block)
        workflow(log_subject: "applying functions") do
          # XXX
          block.call
        end
      end

      def outputting!(&block)
        workflow(log_subject: "writing the spreadsheet") do
          block.call
        end
      end

      def to_s
        "ExecutionContext(filename: #{filename}, state: #{state}, line_number: #{line_number}, row_index: #{row_index}, cell: #{cell})"
      end

      private 

      def log(message)
        return unless @verbose
        # TODO include line_number and other info if we have it
        $stderr.puts message
      end

      def write_to_tmp!(data)
        @tmp.truncate(0)
        @tmp.write(data)
        @tmp.rewind
      end

      # TODO we could add a progress loader here... but hopefully it never gets so slow
      # to warrant that
      def workflow(log_subject:, processing_code_section: false)
        log "Started #{log_subject}"

        if processing_code_section
          @line_number = 1
        else
          @row_index = 0
          @cell_index = 0
          @line_number = @length_of_code_section || 1
        end

        ret = yield

        @cell = nil
        @cell_index = nil
        @row_index = nil
        @line_number = nil

        log "Finished #{log_subject}"

        ret
      end
    end
  end
end
