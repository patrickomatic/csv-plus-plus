# typed: strict
# frozen_string_literal: true

require_relative './file_backer_upper'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to CSV
    class CSV < ::CSVPlusPlus::Writer::Writer
      extend ::T::Sig
      include ::CSVPlusPlus::Writer::FileBackerUpper
      include ::CSVPlusPlus::Writer::Merger

      sig { params(options: ::CSVPlusPlus::Options::FileOptions, position: ::CSVPlusPlus::Runtime::Position).void }
      # @param options [Options::FileOptions]
      # @param position [Runtime::Position]
      def initialize(options, position)
        super(position)

        @reader = ::T.let(::CSVPlusPlus::Reader::CSV.new(options), ::CSVPlusPlus::Reader::CSV)
        @options = options
      end

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # Write a +template+ to CSV
      #
      # @param template [Template] The template to use as input to be written.  It should have been compiled by calling
      #   Compiler#compile_template
      def write(template)
        ::CSV.open(@options.output_filename, 'wb') do |csv|
          @position.map_rows(template.rows) do |row|
            csv << build_row(row)
          end
        end
      end

      sig { override.void }
      # Write a backup of the current spreadsheet.
      def write_backup
        backup_file(@options)
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::String)) }
      # Turn the cell into a CSV-
      def evaluate_cell(cell)
        if (ast = cell.ast)
          "=#{ast.evaluate(@position)}"
        else
          cell.value
        end
      end

      private

      sig { params(row: ::CSVPlusPlus::Row).returns(::T::Array[::T.nilable(::String)]) }
      def build_row(row)
        @position.map_row(row.cells) do |cell, _i|
          merge_cell_value(existing_value: @reader.value_at(cell), new_value: evaluate_cell(cell), options: @options)
        end
      end
    end
  end
end
