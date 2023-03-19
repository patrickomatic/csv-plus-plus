# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function call
    #
    # @attr_reader infix [boolean] Whether or not this function call is infix (X * Y, A + B, etc)
    class FunctionCall < EntityWithArguments
      attr_reader :infix

      # @param id [String] The name of the function
      # @param arguments [Array<Entity>] The arguments to the function
      # @param infix [boolean] Whether the function is infix
      def initialize(id, arguments, infix: false)
        super(:function_call, id:, arguments:)

        @infix = infix
      end

      # @return [String]
      def to_s
        if @infix
          "(#{arguments.join(" #{@id} ")})"
        else
          "#{@id.to_s.upcase}(#{arguments_to_s})"
        end
      end

      # @return [boolean]
      def ==(other)
        super && @id == other.id
      end
    end
  end
end
