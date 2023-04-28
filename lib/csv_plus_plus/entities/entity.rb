# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # All classes that are a part of an AST must implement this interface
    class Entity
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { abstract.params(other: ::BasicObject).returns(::T::Boolean) }
      # Each node in the AST needs to implement #== so we can compare entities for equality
      #
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other); end

      sig do
        abstract.params(position: ::CSVPlusPlus::Runtime::Position).returns(::String)
      end
      # Uses the given +position+ to evaluate itself in the current context
      #
      # @param position [Position] The current runtime
      #
      # @return [::String]
      def evaluate(position); end
    end
  end
end
