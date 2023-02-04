# frozen_string_literal: true

module CSVPlusPlus
  module Language
    module Entities
      # A function call
      class FunctionCall < EntityWithArguments
        # initialize
        def initialize(id, arguments)
          super(:function_call, id:, arguments:)
        end

        # to_s
        def to_s
          "#{@id.to_s.upcase}(#{arguments_to_s})"
        end

        # ==
        def ==(other)
          super && @id == other.id
        end
      end
    end
  end
end
