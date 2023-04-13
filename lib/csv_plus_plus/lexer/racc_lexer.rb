# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Lexer
    # Common methods to be mixed into the Racc parsers
    #
    # @attr_reader tokens [Array]
    module RaccLexer
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      RaccToken =
        ::T.type_alias do
          ::T.any(
            [::String, ::Symbol],
            [::Symbol, ::String],
            [::String, ::String],
            [::Symbol, ::Symbol],
            [::FalseClass, ::FalseClass]
          )
        end
      public_constant :RaccToken

      sig { returns(::T::Array[::CSVPlusPlus::Lexer::RaccLexer::RaccToken]) }
      attr_reader :tokens

      sig { params(tokens: ::T::Array[::CSVPlusPlus::Lexer::RaccLexer::RaccToken]).void }
      # Initialize a lexer instance with an empty +@tokens+
      def initialize(tokens: [])
        @tokens = ::T.let(tokens, ::T::Array[::CSVPlusPlus::Lexer::RaccLexer::RaccToken])
      end

      sig { returns(::T.nilable(::CSVPlusPlus::Lexer::RaccLexer::RaccToken)) }
      # Used by racc to iterate each token
      #
      # @return [Array<(Regexp, Symbol) | (false, false)>]
      def next_token
        @tokens.shift
      end

      sig { params(input: ::T.nilable(::String), runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::T.untyped) }
      # Orchestate the tokenizing, parsing and error handling of parsing input.  Each instance will implement their own
      # +#tokenizer+ method
      #
      # @return [Lexer#return_value] Each instance will define it's own +return_value+ with the result of parsing
      def parse(input, runtime)
        return if input.nil?

        return return_value unless anything_to_parse?(input)

        @runtime = ::T.let(runtime, ::T.nilable(::CSVPlusPlus::Runtime::Runtime))

        tokenize(input, runtime)
        do_parse
        return_value
      rescue ::Racc::ParseError => e
        runtime.raise_formula_syntax_error("Error parsing #{parse_subject}", e.message, wrapped_error: e)
      rescue ::CSVPlusPlus::Error::ModifierValidationError => e
        ::Kernel.raise(::CSVPlusPlus::Error::ModifierSyntaxError.from_validation_error(runtime, e))
      end

      protected

      sig { abstract.params(input: ::String).returns(::T::Boolean) }
      # Is the input even worth parsing? for example we don't want to parse cells unless they're a formula (start
      # with '=')
      #
      # @param input [String]
      #
      # @return [Boolean]
      def anything_to_parse?(input); end

      sig { abstract.returns(::String) }
      # Used for error messages, what is the thing being parsed? ("cell value", "modifier", "code section")
      def parse_subject; end

      sig { abstract.returns(::T.untyped) }
      # The output of the parser
      def return_value; end

      sig { abstract.returns(::CSVPlusPlus::Lexer::Tokenizer) }
      # Returns a +Lexer::Tokenizer+ configured for the given
      def tokenizer; end

      private

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).void }
      def tokenize(input, runtime)
        return if input.nil?

        t = tokenizer.scan(input)

        until t.scanner.empty?
          next if t.matches_ignore?

          return if t.stop?

          t.scan_tokens!
          consume_token(t, runtime)
        end

        @tokens << %i[EOL EOL]
      end

      sig { params(tokenizer: ::CSVPlusPlus::Lexer::Tokenizer, runtime: ::CSVPlusPlus::Runtime::Runtime).void }
      # rubocop:disable Metrics/AbcSize
      def consume_token(tokenizer, runtime)
        if tokenizer.last_token&.token && tokenizer.last_match
          @tokens << [::T.must(tokenizer.last_token).token, ::T.must(tokenizer.last_match)]
        elsif tokenizer.scan_catchall
          @tokens << [::T.must(tokenizer.last_match), ::T.must(tokenizer.last_match)]
        # TODO: checking the +parse_subject+ like this is a little hacky... but we need to know if we're parsing
        # modifiers or code_section (or formulas in a cell)
        elsif parse_subject == 'modifier'
          runtime.raise_modifier_syntax_error("Unable to parse #{parse_subject} starting at", tokenizer.peek)
        else
          runtime.raise_formula_syntax_error("Unable to parse #{parse_subject} starting at", tokenizer.peek)
        end
      end
      # rubocop:enable Metrics/AbcSize
    end
  end
end
