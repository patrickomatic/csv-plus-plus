# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A runtime value. These are values which can be materialized at any point via the +resolve_fn+
    # which takes an ExecutionContext as a param
    #
    # @attr_reader resolve_fn [lambda] A lambda that is called when the runtime value is resolved
    class RuntimeValue < Entity
      extend ::T::Sig

      sig { returns(::T::Array[::T.untyped]) }
      attr_reader :arguments

      sig { returns(::T.proc.params(arg0: ::CSVPlusPlus::Runtime::Runtime).returns(::CSVPlusPlus::Entities::Entity)) }
      attr_reader :resolve_fn

      sig do
        params(
          resolve_fn: ::T.proc.params(arg0: ::CSVPlusPlus::Runtime::Runtime).returns(::CSVPlusPlus::Entities::Entity),
          arguments: ::T::Array[::T.untyped]
        ).void
      end
      # @param resolve_fn [lambda] A lambda that is called when the runtime value is resolved
      # @param arguments [Any] Arguments to the runtime value call
      def initialize(resolve_fn, arguments: [])
        super(::CSVPlusPlus::Entities::Type::RuntimeValue)

        @arguments = arguments
        @resolve_fn = resolve_fn
      end

      sig { override.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param _runtime [Runtime]
      #
      # @return [String]
      def evaluate(_runtime)
        '(runtime value)'
      end
    end
  end
end
