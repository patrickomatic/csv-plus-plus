# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function definition
    #
    # @attr_reader body [Entity] The body of the function.  +body+ can contain variable references
    #   from +@arguments+
    class Function < EntityWithArguments
      attr_reader :body

      # @param id [Symbool, String] the name of the function - what it will be callable by
      # @param arguments [Array<Symbol>]
      # @param body [Entity]
      def initialize(id, arguments, body)
        super(:function, id:, arguments: arguments.map(&:to_sym))

        @body = body
      end

      # @param runtime [Runtime]
      #
      # @return [::String]
      def evaluate(runtime)
        "def #{@id.to_s.upcase}(#{arguments.map(&:to_s).join(', ')}) #{@body.evaluate(runtime)}"
      end

      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        super && @body == other.body
      end
    end
  end
end
