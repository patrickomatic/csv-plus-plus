# frozen_string_literal: true

module CSVPlusPlus
  module Language
    module Entities
      # A reference to a variable
      class Variable < Entity
        # @param id [Symbol] The identifier of the variable
        def initialize(id)
          super(:variable, id:)
        end

        # @return [String]
        def to_s
          "$$#{@id}"
        end

        # @return [boolean]
        def ==(other)
          super && id == other.id
        end
      end
    end
  end
end
