# frozen_string_literal: true

require_relative '../entities'

module CSVPlusPlus
  module Entities
    # A basic building block of the abstract syntax tree (AST)
    #
    # @attr_reader id [Symbol] The identifier of the entity.  For functions this is the function name,
    #   for variables it's the variable name
    # @attr_reader type [Symbol] The type of the entity.  Valid values are defined in +::CSVPlusPlus::Entities::TYPES+
    class Entity
      attr_reader :id, :type

      # @param type [::String, Symbol]
      # @param id [::String, nil]
      def initialize(type, id: nil)
        @type = type.to_sym
        @id = id.downcase.to_sym if id
      end

      # @return [boolean]
      def ==(other)
        self.class == other.class && @type == other.type && @id == other.id
      end

      # Respond to predicates that correspond to types like #boolean?, #string?, etc
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      def method_missing(method_name, *_arguments)
        if method_name =~ /^(\w+)\?$/
          t = ::Regexp.last_match(1)
          a_type?(t) && @type == t.to_sym
        else
          super
        end
      end

      # Respond to predicates by type (entity.boolean?, entity.string?, etc)
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      #
      # @return [boolean]
      def respond_to_missing?(method_name, *_arguments)
        (method_name =~ /^(\w+)\?$/ && a_type?(::Regexp.last_match(1))) || super
      end

      private

      def a_type?(str)
        ::CSVPlusPlus::Entities::TYPES.include?(str.to_sym)
      end
    end

    # An entity that can take other entities as arguments.  Current use cases for this
    # are function calls and function definitions
    #
    # @attr_reader arguments [Array<Entity>] The arguments supplied to this entity.
    class EntityWithArguments < Entity
      attr_reader :arguments

      # @param type [::String, Symbol]
      # @param id [::String]
      # @param arguments [Array<Entity>]
      def initialize(type, id: nil, arguments: [])
        super(type, id:)
        @arguments = arguments
      end

      # @return [boolean]
      def ==(other)
        super && @arguments == other.arguments
      end

      protected

      attr_writer :arguments

      def arguments_to_s
        @arguments.join(', ')
      end
    end
  end
end
