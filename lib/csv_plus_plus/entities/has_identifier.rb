# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # Can be included on any class that has a comparable id
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
