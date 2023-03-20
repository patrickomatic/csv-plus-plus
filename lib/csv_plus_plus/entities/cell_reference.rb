# frozen_string_literal: true

require_relative './ast_builder'
require_relative './entity'

module CSVPlusPlus
  module Entities
    # A reference to a cell
    #
    # @attr_reader cell_reference [String] The cell reference in A1 format
    class CellReference < Entity
      attr_reader :cell_reference

      A1_NOTATION_REGEXP = /(['\w]+!)?\w+:\w+/
      public_constant :A1_NOTATION_REGEXP

      # Create a +CellReference+ to the given indexes
      #
      # @param cell_index [Integer] The current cell index
      # @param row_index [Integer] The current row index
      #
      # @return [CellReference]
      def self.from_index(cell_index:, row_index:)
        return unless row_index || cell_index

        # I can't just extend this class due to circular references :(
        ::Class.new.extend(::CSVPlusPlus::Entities::ASTBuilder).ref(cell_index:, row_index:)
      end

      # Does the given +cell_reference_string+ conform to a valid cell reference?
      #
      # {https://developers.google.com/sheets/api/guides/concepts}
      #
      # @param cell_reference_string [::String] The string to check if it is a valid cell reference (we assume it's in
      #   A1 notation but maybe can support R1C1)
      #
      # @return [boolean]
      def self.valid_cell_reference?(cell_reference_string)
        !(cell_reference_string =~ ::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP).nil?
      end

      # @param cell_reference [String] The cell reference in A1 format
      def initialize(cell_reference)
        super(:cell_reference)

        @cell_reference = cell_reference
      end

      # @return [::String]
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
