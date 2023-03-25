# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Some helpful functions that can be mixed into a class to help building ASTs
    module ASTBuilder
      # Let the current class have functions which can build a given entity by calling it's type.  For example
      # +number(1)+, +variable(:foo)+
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      # @param args [Array] The arguments to create the entity with
      # @param kwargs [Hash] The arguments to create the entity with
      #
      # @return [Entity, #super]
      def method_missing(method_name, *args, **kwargs, &)
        entity_class = ::CSVPlusPlus::Entities::TYPES[method_name.to_sym]
        return super unless entity_class

        entity_class.new(*args, **kwargs, &)
      end

      # Let the current class have functions which can build a given entity by calling it's type.  For example
      # +number(1)+, +variable(:foo)+
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      # @param _arguments [] The arguments to create the entity with
      #
      # @return [Boolean, #super]
      def respond_to_missing?(method_name, *_arguments)
        ::CSVPlusPlus::Entities::TYPES.include?(method_name.to_sym) || super
      end
    end
  end
end
