# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      ##
      # A reference to a cell
      class CellReference < Entity
        attr_reader :cell_reference

        # initialize
        def initialize(cell_reference)
          super(:cell_reference)
          @cell_reference = cell_reference
        end

        # to_s
        def to_s
          @cell_reference
        end
      end
    end
  end
end
