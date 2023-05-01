# typed: strict
# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Entities
    # A runtime value. These are values which can be materialized at any point via the +resolve_fn+ which
    # will provide a value depending on the +Runtime+'s current state.
    class RuntimeValue < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      ResolveFn =
        ::T.type_alias do
          ::T.proc.params(
            position: ::CSVPlusPlus::Runtime::Position,
            arguments: ::T::Enumerable[::CSVPlusPlus::Entities::Entity]
          ).returns(::CSVPlusPlus::Entities::Entity)
        end
      public_constant :ResolveFn

      sig do
        params(resolve_fn: ::CSVPlusPlus::Entities::RuntimeValue::ResolveFn).void
      end
      # @param resolve_fn [lambda] A lambda that is called when the runtime value is resolved
      def initialize(resolve_fn)
        @resolve_fn = resolve_fn
        super()
      end

      sig do
        params(
          position: ::CSVPlusPlus::Runtime::Position,
          arguments: ::T::Enumerable[::CSVPlusPlus::Entities::Entity]
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # Using the +@resolve_fn+, evaluate this runtime value into an +Entity+ that can be later evaluated
      def call(position, arguments)
        @resolve_fn.call(position, arguments)
      end

      sig { override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # Given the current runtime, call +@resolve_fn+ to produce a value
      #
      # @param _position [Position]
      #
      # @return [Entities::Entity]
      def evaluate(_position)
        # TODO: we can do a check on arguments here and make sure that the RuntimeValue is being called
        # with the number of arguments it requires
        # @resolve_fn.call(position, ::T.must(arguments)).evaluate(position)
        '(runtime value)'
      end
    end
  end
end
