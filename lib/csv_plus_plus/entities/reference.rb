# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A reference to something - this is typically either a cell reference (handled by +A1Reference+) or a reference
    # to a variable.
    #
    # One sticking point with the design of this class is that we don't know if a reference is a variable reference
    # unless we look at currently defined variables and see if there is one by that name.  Since all cell references
    # can be a valid variable name, we can't be sure which is which.  So we delegate that decision as late as possible
    # - in #evaluate
    #
    # @attr_reader scoped_to_expand [Expand, nil] If set, the expand in which this variable is scoped to. It cannot be
    #   resolved outside of the given expand.
    class Reference < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig
      include ::CSVPlusPlus::Entities::HasIdentifier

      sig { returns(::T.nilable(::String)) }
      attr_reader :ref

      # sig { returns(::T.nilable(::CSVPlusPlus::A1Reference)) }
      # attr_reader :a1_ref

      sig { params(ref: ::T.nilable(::String), a1_ref: ::T.nilable(::CSVPlusPlus::A1Reference)).void }
      # Either +ref+, +cell_index+ or +row_index+ must be specified.
      #
      # @param ref [Integer, nil] An A1-style cell reference (that will be parsed into it's row/cell indexes).
      # @param a1_ref [Integer, nil] An A1-style cell reference (that will be parsed into it's row/cell indexes).
      def initialize(ref: nil, a1_ref: nil)
        super()

        raise(::ArgumentError, 'Must specify :ref or :a1_ref') unless ref || a1_ref

        @ref = ref
        @a1_ref = a1_ref
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [boolean]
      def ==(other)
        case other
        when self.class
          a1_ref == other.a1_ref
        else
          false
        end
      end

      sig { returns(::CSVPlusPlus::A1Reference) }
      # @return [A1Reference]
      def a1_ref
        @a1_ref ||= ::CSVPlusPlus::A1Reference.new(ref: ::T.must(ref))
      end

      sig { override.params(position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # Get the A1-style cell reference
      #
      # @param position [Position] The current position
      #
      # @return [::String] An A1-style reference
      def evaluate(position)
        # TODO: ugh make to_a1_ref not return a nil
        ref || a1_ref.to_a1_ref(position) || ''
      end

      sig { returns(::T.nilable(::Symbol)) }
      # @return [Symbol]
      def id
        ref && identifier(::T.must(ref).to_sym)
      end
    end
  end
end
