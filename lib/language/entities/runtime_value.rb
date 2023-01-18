# frozen_string_literal: true

module CSVPlusPlus
  module Language
    module Entities
      ##
      # A runtime value
      #
      # These are values which can be materialized at any point via the +resolve_fn+
      # which takes an ExecutionContext as a param
      class RuntimeValue < Entity
        attr_reader :resolve_fn

        # initialize
        def initialize(resolve_fn)
          super(:runtime_value)
          @resolve_fn = resolve_fn
        end

        # to_s
        def to_s
          '(runtime_value)'
        end
      end
    end
  end
end
