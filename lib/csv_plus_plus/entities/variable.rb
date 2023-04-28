# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # TODO: get rid of this I think - everything will just be References
    #
    # A reference to a variable
    class Variable < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig
      include ::CSVPlusPlus::Entities::HasIdentifier

      sig { returns(::Symbol) }
      attr_reader :id

      sig { params(id: ::Symbol).void }
      # @param id [Symbol] The identifier of the variable
      def initialize(id)
        super()

        @id = ::T.let(identifier(id), ::Symbol)
      end

      sig { override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param _position [Position]
      #
      # @return [::String]
      def evaluate(_position)
        "$$#{@id}"
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        case other
        when self.class
          # XXX move this into HasIdentifier
          @id == other.id
        else
          false
        end
      end
    end
  end
end
