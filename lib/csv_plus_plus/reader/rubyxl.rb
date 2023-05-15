# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Reader
    # Reads an Excel file
    class RubyXL < ::CSVPlusPlus::Reader::Reader
      extend ::T::Sig
      extend ::T::Generic

      CellValue = type_member { { fixed: ::RubyXL::Cell } }
      public_constant :CellValue

      sig { params(options: ::CSVPlusPlus::Options::FileOptions, worksheet: ::RubyXL::Worksheet).void }
      # Open an excel outputter to the +output_filename+ specified by the +Options+
      #
      # @param options [Options] The supplied options.
      # @param worksheet [RubyXL::Worksheet] The already-opened RubyXL worksheet
      def initialize(options, worksheet)
        super()

        @options = options
        @worksheet = worksheet
      end

      sig { override.params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::CSVPlusPlus::Reader::RubyXL::CellValue)) }
      # Get the current value at the +cell+'s position
      #
      # @param cell [Cell]
      #
      # @return [RubyXL::Cell, nil]
      def value_at(cell)
        @worksheet.sheet_data[cell.row_index]&.[](cell.index)
      end
    end
  end
end
