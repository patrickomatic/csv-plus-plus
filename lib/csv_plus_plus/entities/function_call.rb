# typed: true
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

      # @param runtime [Runtime]
      #
      # @return [::String]
      def evaluate(runtime)
        evaluated_arguments = evaluate_arguments(runtime)

        if @infix
          "(#{evaluated_arguments.join(" #{@id} ")})"
        else
          "#{@id.to_s.upcase}(#{evaluated_arguments.join(', ')})"
        end
      end

      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        super && @id == other.id && @infix == other.infix
      end
    end
  end
end
