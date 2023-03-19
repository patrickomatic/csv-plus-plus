# frozen_string_literal: true

require_relative '../ast_builder'
require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      # A reference to a cell
      #
      # @attr_reader cell_reference [String] The cell reference in A1 format
      class CellReference < Entity
        attr_reader :cell_reference

        # Create a +CellReference+ to the given indexes
        #
        # @param cell_index [Integer] The current cell index
        # @param row_index [Integer] The current row index
        #
        # @return [CellReference]
        def self.from_index(cell_index:, row_index:)
          return unless row_index || cell_index

          # I can't just extend this class due to circular references
          # new(Class.new.extend(::CSVPlusPlus::Language::ASTBuilder).ref(cell_index:, row_index:))
        end

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
