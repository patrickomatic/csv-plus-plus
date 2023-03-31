# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A date value
    #
    # @attr_reader value [Date] The parsed date
    class Date < Entity
      attr_reader :value

      # TODO: support time?
      DATE_STRING_REGEXP = %r{^\d{1,2}[/-]\d{1,2}[/-]\d{1,4}?$}
      public_constant :DATE_STRING_REGEXP

      # Is the given string a valid date?
      #
      # @param date_string [::String]
      def self.valid_date?(date_string)
        !(date_string.strip =~ ::CSVPlusPlus::Entities::Date::DATE_STRING_REGEXP).nil?
      end

      # @param value [String] The user-inputted date value
      def initialize(value)
        super(:date)

        @value = ::Date.parse(value)
      end

      # @param _runtime [Runtime]
      #
      # @return [::String]
      def evaluate(_runtime)
        @value
      end
    end
  end
end
