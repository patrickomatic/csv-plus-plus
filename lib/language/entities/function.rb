# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    module Entities
      ##
      # A static function definition
      class Function < EntityWithArguments
        attr_reader :body

        # initialize
        def initialize(id, arguments, body)
          super(:function, id:, arguments: arguments.map(&:to_sym))
          @body = body
        end

        # to_s
        def to_s
          @id.to_s.upcase
        end

        # ==
        def ==(other)
          super && @body == other.body
        end
      end
    end
  end
end
