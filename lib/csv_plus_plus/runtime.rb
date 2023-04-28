# typed: strict
# frozen_string_literal: true

require_relative './runtime/graph'
require_relative './runtime/position'
require_relative './runtime/references'
require_relative './runtime/runtime'
require_relative './runtime/scope'

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
        position: ::T.nilable(::CSVPlusPlus::Runtime::Position),
        scope: ::T.nilable(::CSVPlusPlus::Runtime::Scope)
      ).returns(::CSVPlusPlus::Runtime::Runtime)
    end
    # Initialize a runtime instance with all the functionality we need.  A runtime is one-to-one with a file being
    # compiled.
    #
    # @param source_code [SourceCode] The csv++ source code to be compiled
    # @param position [Position, nil]
    # @param scope [Scope, nil]
    #
    # @return [Runtime::Runtime]
    def self.new(source_code:, position: nil, scope: nil)
      position ||= ::CSVPlusPlus::Runtime::Position.new(source_code.input)
      ::CSVPlusPlus::Runtime::Runtime.new(source_code:, position:, scope:)
    end
  end
end
