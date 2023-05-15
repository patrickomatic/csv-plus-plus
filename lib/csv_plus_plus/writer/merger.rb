# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # A merging strategy for when we want to write to a cell but it has a value
    module Merger
      extend ::T::Sig
      include ::Kernel

      sig do
        type_parameters(:V)
          .params(
            existing_value: ::T.nilable(::T.all(::T.type_parameter(:V), ::BasicObject)),
            new_value: ::T.nilable(::T.all(::T.type_parameter(:V), ::BasicObject)),
            options: ::CSVPlusPlus::Options::Options
          ).returns(::T.nilable(::T.type_parameter(:V)))
      end
      # Consistently enforce our strategy for resolving differences between new changes and existing. By default we
      # overwrite values that are currently in the spreadsheet but you can override that with the --safe flag
      def merge_cell_value(existing_value:, new_value:, options:)
        merged_value = merge_with_strategy(existing_value:, new_value:, options:)

        return merged_value unless options.verbose

        if options.overwrite_values && merged_value != existing_value
          warn("Overwriting existing value: \"#{existing_value}\" with \"#{new_value}\"")
        # rubocop:disable Style/MissingElse
        elsif !options.overwrite_values && new_value != merged_value
          # rubocop:enable Style/MissingElse
          warn("Keeping old value: \"#{existing_value}\" rather than new value: \"#{new_value}\"")
        end

        merged_value
      end

      private

      sig do
        type_parameters(:V)
          .params(
            existing_value: ::T.nilable(::T.all(::T.type_parameter(:V), ::BasicObject)),
            new_value: ::T.nilable(::T.all(::T.type_parameter(:V), ::BasicObject)),
            options: ::CSVPlusPlus::Options::Options
          ).returns(::T.nilable(::T.type_parameter(:V)))
      end
      def merge_with_strategy(existing_value:, new_value:, options:)
        if options.overwrite_values
          new_value || existing_value
        else
          existing_value || new_value
        end
      end
    end
  end
end
