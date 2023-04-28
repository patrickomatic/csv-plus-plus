# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A runtime value. These are values which can be materialized at any point via the +resolve_fn+ which
    # will provide a value depending on the +Runtime+'s current state.
    module HasIdentifier
      extend ::T::Sig

      sig { params(symbol: ::Symbol).returns(::Symbol) }
      # Variables and functions are case insensitive. I hate it but it's how excel is
      #
      # @param symbol [Symbol]
      def identifier(symbol)
        symbol.downcase
      end
    end
  end
end
