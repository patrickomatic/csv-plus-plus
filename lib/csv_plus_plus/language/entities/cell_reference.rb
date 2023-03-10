# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      # A reference to a cell
      #
      # @attr_reader cell_reference [String] The cell reference in A1 format
      class CellReference < Entity
        attr_reader :cell_reference

        # @param cell_reference [String] The cell reference in A1 format
        def initialize(cell_reference)
          super(:cell_reference)

          @cell_reference = cell_reference
        end

        # @return [String]
        def to_s
          @cell_reference
        end

        # @return [Boolean]
        def ==(other)
          super && @cell_reference == other.cell_reference
        end
      end
    end
  end
end
