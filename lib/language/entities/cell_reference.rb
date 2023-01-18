# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      ##
      # A reference to a cell
      class CellReference < Entity
        # initialize
        def initialize(id)
          super(:cell_reference, id:)
        end

        # to_s
        def to_s
          @id.to_s.upcase
        end
      end
    end
  end
end
