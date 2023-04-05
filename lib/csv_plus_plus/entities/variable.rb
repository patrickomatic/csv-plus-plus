# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # TODO: get rid of this I think - everything will just be References
    #
    # A reference to a variable
    class Variable < Entity
      extend ::T::Sig

      sig { params(id: ::Symbol).void }
      # @param id [Symbol] The identifier of the variable
      def initialize(id)
        super(::CSVPlusPlus::Entities::Type::Variable, id:)
      end

      sig { override.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param _runtime [Runtime]
      #
      # @return [::String]
      def evaluate(_runtime)
        "$$#{@id}"
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && @id == other.id
      end
    end
  end
end
