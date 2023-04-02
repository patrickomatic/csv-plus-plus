# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # The logic for how a row can expand
    #
    # @attr_reader ends_at [Integer, nil] Once the row has been expanded, where it ends at.
    # @attr_reader repetitions [Integer, nil] How many times the row repeats/expands.
    # @attr_reader starts_at [Integer, nil] Once the row has been expanded, where it starts at.
    class Expand
      extend ::T::Sig

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :ends_at

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :repetitions

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :starts_at

      sig { params(repetitions: ::T.nilable(::Integer), starts_at: ::T.nilable(::Integer)).void }
      # @param repetitions [Integer, nil] How many times this expand repeats.  If it's +nil+ it will expand infinitely
      #   (for the rest of the worksheet.)
      # @param starts_at [Integer, nil] The final location where the +Expand+ will start.  It's important to note that
      #   this can't be derived until all rows are expanded, because each expand modifier will push down the ones below
      #   it.  So typically this param will not be passed in the initializer but instead set later.
      def initialize(repetitions: nil, starts_at: nil)
        @repetitions = ::T.let(repetitions, ::T.nilable(::Integer))
        @starts_at = ::T.let(starts_at, ::T.nilable(::Integer)) unless starts_at.nil?
        @ends_at = ::T.let(nil, ::T.nilable(::Integer))
      end

      sig { returns(::T::Boolean) }
      # Has the row been expanded?
      #
      # @return [boolean]
      def expanded?
        !@starts_at.nil?
      end

      sig { returns(::T::Boolean) }
      # Does this infinitely expand?
      #
      # @return [boolean]
      def infinite?
        repetitions.nil?
      end

      sig { params(row_index: ::Integer).void }
      # Mark the start of the row once it's been expanded, as well as where it +ends_at+.  When expanding rows each one
      # adds rows to the worksheet and if there are multiple `expand` modifiers in the worksheet, we don't know the
      # final +row_index+ until we're in the phase of expanding all the rows out.
      def starts_at=(row_index)
        @starts_at = row_index
        @ends_at = row_index + @repetitions unless @repetitions.nil?
      end
    end
  end
end
