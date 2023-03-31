# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # A class that handles the rules for modifiers to support conditional formatting.
    class ConditionalFormatting
      attr_reader :arguments, :condition, :invalid_reason

      # @param value [::String] The unparsed conditional formatting rule
      def initialize(value)
        condition, args = value.split(/\si:\s*/)
        @condition = condition.to_sym
        @arguments = args.split(/\s+/)
      end
    end
  end
end
