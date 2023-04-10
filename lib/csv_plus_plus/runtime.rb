# typed: strict
# frozen_string_literal: true

require_relative './runtime/can_define_references'
require_relative './runtime/can_resolve_references'
require_relative './runtime/graph'
require_relative './runtime/position_tracker'
require_relative './runtime/references'
require_relative './runtime/runtime'

module CSVPlusPlus
  # All functionality needed to keep track of the runtime AKA execution context.  This module has a lot of
  # reponsibilities:
  #
  # - variables and function resolution and scoping
  # - variable & function definitions
  # - keeping track of the runtime state (the current cell being processed)
  # - rewriting the input file that's being parsed
  #
  module Runtime
    extend ::T::Sig

    sig do
      params(
        source_code: ::CSVPlusPlus::SourceCode,
        functions: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function],
        variables: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]
      ).returns(::CSVPlusPlus::Runtime::Runtime)
    end
    # Initialize a runtime instance with all the functionality we need.  A runtime is one-to-one with a file being
    # compiled.
    #
    # @param source_code [SourceCode] The csv++ source code to be compiled
    # @param functions [Hash<Symbol, Function>] Pre-defined functions
    # @param variables [Hash<Symbol, Entity>] Pre-defined variables
    #
    # @return [Runtime::Runtime]
    def self.new(source_code:, functions: {}, variables: {})
      ::CSVPlusPlus::Runtime::Runtime.new(source_code:, functions:, variables:)
    end
  end
end
