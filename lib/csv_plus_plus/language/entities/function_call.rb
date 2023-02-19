# frozen_string_literal: true

module CSVPlusPlus
  module Language
    module Entities
      # A function call
      class FunctionCall < EntityWithArguments
        # @param id [String] The name of the function
        # @param arguments [Array<Entity>] The arguments to the function
        def initialize(id, arguments)
          super(:function_call, id:, arguments:)
        end

        # @return [String]
        def to_s
          "#{@id.to_s.upcase}(#{arguments_to_s})"
        end

        # @return [boolean]
        def ==(other)
          super && @id == other.id
        end
      end
    end
  end
end
