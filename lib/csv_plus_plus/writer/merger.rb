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
      # Our strategy for resolving differences between new changes and existing
      def merge_cell_value(existing_value:, new_value:, options:)
        # TODO: make an option that specifies if we override (take new data over old)
        merged_value = new_value || existing_value

        return merged_value if !options.verbose || merged_value == existing_value

        warn("Overwriting existing value: \"#{existing_value}\" with \"#{new_value}\"")
        merged_value
      end
    end
  end
end
