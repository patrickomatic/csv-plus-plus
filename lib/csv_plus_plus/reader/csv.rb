# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Reader
    # Reads a CSV file
    class CSV < ::CSVPlusPlus::Reader::Reader
      extend ::T::Sig
      extend ::T::Generic

      CellValue = type_member { { fixed: ::String } }
      public_constant :CellValue

      sig { params(options: ::CSVPlusPlus::Options::FileOptions).void }
      # Open a CSV outputter to the +output_filename+ specified by the +Options+
      #
      # @param options [Options] The supplied options.
      def initialize(options)
        super()

        @options = options
        @cell_values = ::T.let(
          read_csv(@options.output_filename),
          ::T::Array[::T::Array[::T.nilable(::CSVPlusPlus::Reader::CSV::CellValue)]]
        )
      end

      sig { override.params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::CSVPlusPlus::Reader::CSV::CellValue)) }
      # Get the current value at the +cell+'s location.
      #
      # @param cell [Cell]
      #
      # @return [CellValue, nil]
      def value_at(cell)
        @cell_values[cell.row_index]&.[](cell.index)
      end

      protected

      sig do
        params(filename: ::Pathname).returns(::T::Array[::T::Array[::T.nilable(::CSVPlusPlus::Reader::CSV::CellValue)]])
      end
      def read_csv(filename)
        return [[]] unless ::File.exist?(filename)

        ::CSV.read(filename.to_s)
      end
    end
  end
end
