# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Some helpful functions that can be mixed into a class to help building ASTs
    module ASTBuilder
      # Let the current class have functions which can build a given entity by calling it's type.  For example
      # +number(1)+, +variable(:foo)+
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      # @param arguments [] The arguments to create the entity with
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
      # @param arguments [] The arguments to create the entity with
      #
      # @return [Boolean, #super]
      def respond_to_missing?(method_name, *_arguments)
        ::CSVPlusPlus::Entities::TYPES.include?(method_name.to_sym) || super
      end

      # Turns index-based/X,Y coordinates into a A1 format
      #
      # @param row_index [Integer]
      # @param cell_index [Integer]
      #
      # @return [String]
      def ref(row_index: nil, cell_index: nil)
        return unless row_index || cell_index

        rowref = row_index ? (row_index + 1).to_s : ''
        cellref = cell_index ? cell_ref(cell_index) : ''
        cell_reference([cellref, rowref].join)
      end

      private

      ALPHA = ('A'..'Z').to_a.freeze
      private_constant :ALPHA

      def cell_ref(cell_index)
        c = cell_index.dup
        ref = ''

        while c >= 0
          # rubocop:disable Lint/ConstantResolution
          ref += ALPHA[c % 26]
          # rubocop:enable Lint/ConstantResolution
          c = (c / 26).floor - 1
        end

        ref.reverse
      end
    end
  end
end
