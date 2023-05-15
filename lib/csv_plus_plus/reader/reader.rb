# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Reader
    # +Reader+s are used to complement +Writer+ instances by reading in the existing spreadsheet and providing
    # a way to merge in results.
    class Reader
      extend ::T::Sig
      extend ::T::Generic
      extend ::T::Helpers

      abstract!

      CellValue = type_member
      public_constant :CellValue

      sig { abstract.params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::CSVPlusPlus::Reader::Reader::CellValue)) }
      # Get the current value at the +cell+'s location.
      #
      # @param cell [Cell]
      #
      # @return [CellValue, nil]
      def value_at(cell); end
    end
  end
end
