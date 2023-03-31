# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # A validation on a cell value.  Used to support the `validate=` modifier directive.  This is mostly based on the
    # Google Sheets API spec which can be seen here:
    #
    # {https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#ConditionType}
    #
    # @attr_reader arguments [Array<::String>] The parsed arguments as required by the condition.
    # @attr_reader condition [Symbol] The condition (:blank, :text_eq, :date_before, etc.)
    # @attr_reader invalid_reason [::String, nil] If set, the reason why this modifier is not valid.
    class DataValidation
      attr_reader :arguments, :condition, :invalid_reason

      # @param value [::String] The value to parse as a data validation
      def initialize(value)
        condition, args = value.split(/\s*:\s*/)
        @arguments = unquote(args || '').split(/\s+/)
        @condition = condition.to_sym

        validate!
      end

      # Each data validation (represented by +@condition+) has their own requirements for which arguments are valid.
      # If this object is invalid, you can see the reason in +@invalid_reason+.
      #
      # @return [boolean]
      def valid?
        @invalid_reason.nil?
      end

      # @return [::String]
      def to_s
        "DataValidation(arguments: #{@arguments}, condition: #{@condition}, invalid_reason: #{@invalid_reason})"
      end

      protected

      def unquote(str)
        # TODO: I'm pretty sure this isn't sufficient and we need to deal with the backslashes
        str.gsub(/^['\s]*|['\s]*$/, '')
      end

      def invalid!(reason)
        @invalid_reason = reason
      end

      def a_number(arg)
        Float(arg)
      rescue ::ArgumentError
        invalid!("Requires a number but given: #{arg}")
      end

      def a1_notation(arg)
        return arg if ::CSVPlusPlus::Entities::CellReference.valid_cell_reference?(arg)
      end

      def a_date(arg, allow_relative_date: false)
        return arg if ::CSVPlusPlus::Entities::Date.valid_date?(arg)

        if allow_relative_date
          a_relative_date(arg)
        else
          invalid!("Requires a date but given: #{arg}")
        end
      end

      def a_relative_date(arg)
        return arg if %w[past_month past_week past_year yesterday today tomorrow].include?(arg.downcase)

        invalid!('Requires a relative date: past_month, past_week, past_year, yesterday, today or tomorrow')
      end

      def no_args
        return if @arguments.empty?

        invalid!("Requires no arguments but #{@arguments.length} given: #{@arguments}")
      end

      def one_arg
        return @arguments[0] if @arguments.length == 1

        invalid!("Requires only one argument but #{@arguments.length} given: #{@arguments}")
      end

      def one_arg_or_more
        return @arguments if @arguments.length.positive?

        invalid!("Requires at least one argument but #{@arguments.length} given: #{@arguments}")
      end

      def two_dates
        return @arguments if @arguments.length == 2 && a_date(@arguments[0]) && a_date(@arguments[1])

        invalid!("Requires exactly two dates but given: #{@arguments}")
      end

      def two_numbers
        return @arguments if @arguments.length == 2 && a_number(@arguments[0]) && a_number(@arguments[1])

        invalid!("Requires exactly two numbers but given: #{@arguments}")
      end

      # validate_boolean is a weird one because it can have 0, 1 or 2 @arguments - all of them must be (true | false)
      def validate_boolean
        return @arguments if @arguments.empty?

        converted_args = @arguments.map(&:strip).map(&:downcase)
        return @arguments if [1, 2].include?(@arguments.length) && converted_args.all? do |arg|
                               %w[true false].include?(arg)
                             end

        invalid!("Requires 0, 1 or 2 arguments and they all must be either 'true' or 'false'. Received: #{arguments}")
      end

      # rubocop:disable Metrics/MethodLength, Metrics/CyclomaticComplexity, Metrics/AbcSize
      def validate!
        case condition.to_sym
        when :blank, :date_is_valid, :not_blank, :text_is_email, :text_is_url
          no_args
        when :text_contains, :text_ends_with, :text_eq, :text_not_contains, :text_starts_with
          one_arg
        when :date_after, :date_before, :date_on_or_after, :date_on_or_before
          a_date(one_arg, allow_relative_date: true)
        when :date_eq, :date_not_eq
          a_date(one_arg)
        when :date_between, :date_not_between
          two_dates
        when :one_of_range
          a1_notation(one_arg)
        when :custom_formula, :one_of_list, :text_not_eq
          one_arg_or_more
        when :number_eq, :number_greater, :number_greater_than_eq, :number_less, :number_less_than_eq, :number_not_eq
          a_number(one_arg)
        when :number_between, :number_not_between
          two_numbers
        when :boolean
          validate_boolean
        else
          invalid!('Not a recognized data validation directive')
        end
      end
      # rubocop:enable Metrics/MethodLength, Metrics/CyclomaticComplexity, Metrics/AbcSize
    end
  end
end
