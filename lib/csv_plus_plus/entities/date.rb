# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A date value
    #
    # @attr_reader value [Date] The parsed date
    class Date < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      sig { returns(::Date) }
      attr_reader :value

      # TODO: support time granularity?
      DATE_STRING_REGEXP = %r{^\d{1,2}[/-]\d{1,2}[/-]\d{1,4}?$}
      public_constant :DATE_STRING_REGEXP

      sig { params(date_string: ::String).returns(::T::Boolean) }
      # Is the given string a valid date?
      #
      # @param date_string [::String]
      def self.valid_date?(date_string)
        new(date_string)
        true
      rescue ::Date::Error
        false
      end

      sig { params(value: ::String).void }
      # @param value [::String] The user-inputted date value
      def initialize(value)
        super()

        parsed =
          begin
            ::Date.parse(value)
          rescue ::Date::Error
            ::Date.strptime(value, '%d/%m/%yyyy')
          end
        @value = ::T.let(parsed, ::Date)
      end

      sig { override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param _position [Position]
      #
      # @return [::String]
      def evaluate(_position)
        @value.strftime('%m/%d/%y')
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [Boolean]
      def ==(other)
        case other
        when self.class
          other.value == @value
        else
          false
        end
      end
    end
  end
end
