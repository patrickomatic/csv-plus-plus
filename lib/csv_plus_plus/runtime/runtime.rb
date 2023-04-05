# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # The runtime state of the compiler (the current +line_number+/+row_index+, +cell+ being processed, etc) for parsing
    # a given file.  We take multiple runs through the input file for parsing so it's really convenient to have a
    # central place for these things to be managed.
    #
    # @attr_reader filename [String, nil] The filename that the input came from (mostly used for debugging since
    #   +filename+ can be +nil+ if it's read from stdin.
    #
    # @attr cell [Cell] The current cell being processed
    # @attr cell_index [Integer] The index of the current cell being processed (starts at 0)
    # @attr row_index [Integer] The index of the current row being processed (starts at 0)
    # @attr line_number [Integer] The line number of the original csvpp template (starts at 1)
    class Runtime
      extend ::T::Sig

      include ::CSVPlusPlus::Runtime::CanDefineReferences
      include ::CSVPlusPlus::Runtime::CanResolveReferences
      include ::CSVPlusPlus::Runtime::PositionTracker

      sig { returns(::String) }
      attr_reader :filename

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function]) }
      attr_reader :functions

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]) }
      attr_reader :variables

      sig { returns(::T.nilable(::CSVPlusPlus::Cell)) }
      attr_accessor :cell

      sig do
        params(
          input: ::String,
          filename: ::T.nilable(::String),
          functions: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function],
          variables: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]
        ).void
      end
      # @param input [String] The input to be parsed
      # @param filename [String, nil] The filename that the input came from (mostly used for debugging since +filename+
      #   can be +nil+ if it's read from stdin
      # @param functions [Hash<Symbol, Function>] Pre-defined functions
      # @param variables [Hash<Symbol, Entity>] Pre-defined variables
      def initialize(input:, filename: nil, functions: {}, variables: {})
        @filename = ::T.let(filename || 'stdin', ::String)
        @functions = functions
        @variables = variables

        init_input!(input)
      end

      sig { params(fn_id: ::Symbol).returns(::T::Boolean) }
      # Is +fn_id+ a builtin function?
      #
      # @param var_id [Symbol] The Function#id to check if it's a runtime variable
      #
      # @return [T::Boolean]
      def builtin_function?(fn_id)
        ::CSVPlusPlus::Entities::Builtins::FUNCTIONS.key?(fn_id)
      end

      sig { params(var_id: ::Symbol).returns(::T::Boolean) }
      # Is +var_id+ a builtin variable?
      #
      # @param var_id [Symbol] The Variable#id to check if it's a runtime variable
      #
      # @return [T::Boolean]
      def builtin_variable?(var_id)
        ::CSVPlusPlus::Entities::Builtins::VARIABLES.key?(var_id)
      end

      sig do
        params(message: ::String, bad_input: ::String, wrapped_error: ::T.nilable(::StandardError))
          .returns(::T.noreturn)
      end
      # Called when an error is encoutered during parsing formulas (whether in the code section or a cell).  It will
      # construct a useful error with the current +@row/@cell_index+, +@line_number+ and +@filename+
      #
      # @param message [::String] A message relevant to why this error is being raised.
      # @param bad_input [::String] The offending input that caused this error to be thrown.
      # @param wrapped_error [StandardError, nil] The underlying error that was raised (if it's not from our own logic)
      def raise_formula_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Error::FormulaSyntaxError.new(message, bad_input, self, wrapped_error:))
      end

      sig do
        params(message: ::String, bad_input: ::String, wrapped_error: ::T.nilable(::StandardError))
          .returns(::T.noreturn)
      end
      # Called when an error is encountered while parsing a modifier.
      #
      # @param message [::String] A message relevant to why this error is being raised.
      # @param bad_input [::String] The offending input that caused this error to be thrown.
      # @param wrapped_error [StandardError, nil] The underlying error that was raised (if it's not from our own logic)
      def raise_modifier_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Error::ModifierSyntaxError.new(self, bad_input:, message:, wrapped_error:))
      end
    end
  end
end
