# frozen_string_literal: true

require 'tempfile'
require_relative 'entities'
require_relative 'global_scope'

module CSVPlusPlus
  module Language
    ##
    # Encapsulates the runtime of a processing of a csvpp file
    # rubocop:disable Metrics/ClassLength
    class ExecutionContext
      attr_reader :filename, :global_scope
      attr_accessor :verbose, :cell, :cell_index, :row_index, :line_number

      # Create an execution context and make sure it gets cleaned up
      def self.with_execution_context(input:, filename:, verbose: false)
        ec = new(input:, filename:, verbose:)
        yield(ec)
      rescue ::StandardError
        ec.unlink!
      end

      # initialize
      def initialize(input:, filename:, verbose:)
        @filename = filename || 'stdin'
        @verbose = verbose
        @length_of_code_section = nil
        @length_of_original_file = input.count("\n")

        # we're gonna take our input file, write it to a tmp file then each
        # step is gonna mutate that tmp file
        @tmp = ::Tempfile.new
        write_to_tmp!(input)
      end

      # cleanup
      def unlink!
        return unless @tmp

        @tmp.close
        @tmp.unlink
        @tmp = nil
      end

      # map over an unparsed file and keep track of line_number and row_index
      def map_lines(lines, &block)
        lines.map do |line|
          block.call(line).tap do
            @row_index += 1 unless @row_index.nil?
            @line_number += 1
          end
        end
      end

      # map over a single row and keep track of the cell and it's index
      def map_row(row, &block)
        row.map.with_index do |cell, index|
          @cell = cell
          @cell_index = index
          block.call(@cell, index)
        end
      end

      # map over all rows and keep track of row and line numbers
      # rubocop:disable Metrics/MethodLength
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
      # rubocop:enable Metrics/MethodLength

      # workflow when parsing the code section
      def parsing_code_section!(&block)
        workflow(
          log_subject: 'parsing code section',
          processing_code_section: true
        ) do
          csv_section = block.call(@tmp)
          write_to_tmp!(csv_section)

          @length_of_csv_section = csv_section.count("\n")
          @length_of_code_section = @length_of_original_file - @length_of_csv_section
        end
      end

      # workflow when parsing csv
      def parsing_csv!(&block)
        workflow(log_subject: 'parsing CSV section') do
          block.call(@tmp)
        end

        # we're done with the file and everything is in memory
        unlink!
      end

      # workflow when expanding rows
      def expanding!(&block)
        workflow(log_subject: 'expanding rows') do
          block.call
        end
      end

      # workflow when resolving static variable definitions
      def resolve_static_variables!(code_section)
        # TODO: this indirection seems kinda unnecessary
        @global_scope = ::CSVPlusPlus::Language::GlobalScope.new(code_section)
        @global_scope.resolve_static_variables(code_section.variables, self)
      end

      # workflow when resolving the values of all cells
      def resolve_all_cells!(code_section, rows)
        resolve_static_variables!(code_section) unless @global_scope

        workflow(log_subject: 'resolving all cell value variable references') do
          map_rows(rows, cells_too: true) do |cell|
            cell.ast = @global_scope.resolve_cell_value(self) if cell.ast
          end
        end
      end

      # workflow when resolving functions
      def applying_functions!(&block)
        workflow(log_subject: 'applying functions') do
          # XXX
          block.call
        end
      end

      # workflow when writing results
      def outputting!(&block)
        workflow(log_subject: 'writing the spreadsheet') do
          block.call
        end
      end

      # to_s
      def to_s
        "ExecutionContext(filename: #{filename}, state: #{state}, line_number: #{line_number}, " \
          "row_index: #{row_index}, cell: #{cell})"
      end

      private

      def log(message)
        return unless @verbose

        # TODO: include line_number and other info if we have it
        warn(message)
      end

      def write_to_tmp!(data)
        @tmp.truncate(0)
        @tmp.write(data)
        @tmp.rewind
      end

      # TODO: we could add a progress loader here... but hopefully it never gets so slow
      # to warrant that
      # rubocop:disable Metrics/MethodLength
      def workflow(log_subject:, processing_code_section: false)
        log("Started #{log_subject}")

        if processing_code_section
          @line_number = 1
        else
          @row_index = @cell_index = 0
          @line_number = @length_of_code_section || 1
        end

        ret = yield

        @cell = @cell_index = @row_index = @line_number = nil

        log("Finished #{log_subject}")

        ret
      end
      # rubocop:enable Metrics/MethodLength
    end
    # rubocop:enable Metrics/ClassLength
  end
end
