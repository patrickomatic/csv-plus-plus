# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A runtime value. These are values which can be materialized at any point via the +resolve_fn+
    # which takes an ExecutionContext as a param
    #
    # @attr_reader resolve_fn [lambda] A lambda that is called when the runtime value is resolved
    class RuntimeValue < Entity
      attr_reader :arguments, :resolve_fn

      # @param resolve_fn [lambda] A lambda that is called when the runtime value is resolved
      # @param arguments [Any] Arguments to the runtime value call
      def initialize(resolve_fn, arguments: nil)
        super(:runtime_value)

        @arguments = arguments
        @resolve_fn = resolve_fn
      end

      # @param _runtime [Runtime]
      #
      # @return [String]
      def evaluate(_runtime)
        '(runtime value)'
      end
    end
  end
end
