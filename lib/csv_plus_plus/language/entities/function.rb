# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      # A function definition
      class Function < EntityWithArguments
        attr_reader :body

        # Create a function
        # @param id [Symbool, String] the name of the function - what it will be callable by
        # @param arguments [Array(Symbol)]
        # @param body [Entity]
        def initialize(id, arguments, body)
          super(:function, id:, arguments: arguments.map(&:to_sym))
          @body = body
        end

        # to_s
        def to_s
          "def #{@id.to_s.upcase}(#{arguments_to_s}) #{@body}"
        end

        # ==
        def ==(other)
          super && @body == other.body
        end
      end
    end
  end
end
