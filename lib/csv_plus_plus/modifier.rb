# typed: true
# frozen_string_literal: true

require_relative './modifier/conditional_formatting'
require_relative './modifier/data_validation'
require_relative './modifier/modifier'
require_relative './modifier/validated_modifier'

module CSVPlusPlus
  # All modifier-specific logic is hidden in this module and callers should just call +#new+ on this module.
  module Modifier
    # Return a +Modifier+ with the proper validation and helper functions attached
    #
    # @param row_level [boolean] is this a row level modifier? (otherwise cell-level)
    #
    # @return [ValidatedModifier]
    def self.new(row_level: false)
      ::CSVPlusPlus::Modifier::ValidatedModifier.new(row_level:)
    end
  end
end
