# frozen_string_literal: true

module CSVPlusPlus
  module Language
    module Entities
      ##
      # A reference to a variable
      class Variable < Entity
        # initialize
        def initialize(id)
          super(:variable, id:)
        end

        # to_s
        def to_s
          "$$#{@id}"
        end

        # ==
        def ==(other)
          super || id == other.id
        end
      end
    end
  end
end
