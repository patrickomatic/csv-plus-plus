# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # A runtime value. These are values which can be materialized at any point via the +resolve_fn+ which
    # will provide a value depending on the +Runtime+'s current state.
    class Value
      extend ::T::Sig

      ResolveFn =
        ::T.type_alias do
          ::T.proc.params(
            runtime: ::CSVPlusPlus::Runtime::Runtime,
            arguments: ::T::Array[::CSVPlusPlus::Entities::Entity]
          ).returns(::CSVPlusPlus::Entities::Entity)
        end
      public_constant :ResolveFn

      sig do
        params(resolve_fn: ::CSVPlusPlus::Runtime::Value::ResolveFn).void
      end
      # @param resolve_fn [lambda] A lambda that is called when the runtime value is resolved
      def initialize(resolve_fn)
        @resolve_fn = resolve_fn
      end

      sig do
        params(
          runtime: ::CSVPlusPlus::Runtime::Runtime,
          arguments: ::T::Array[::CSVPlusPlus::Entities::Entity]
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # Given the current runtime, call +@resolve_fn+ to produce a value
      #
      # @param runtime [Runtime]
      # @param arguments [Array<Entity>]
      #
      # @return [Entities::Entity]
      def call(runtime, arguments)
        @resolve_fn.call(runtime, arguments)
      end
    end
  end
end
