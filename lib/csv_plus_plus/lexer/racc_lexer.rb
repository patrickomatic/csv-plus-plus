# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Lexer
    # TODO: ugh clean this up
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

    # Common methods to be mixed into the Racc parsers
    #
    # @attr_reader tokens [Array]
    module RaccLexer
      extend ::T::Sig
      extend ::T::Helpers
      extend ::T::Generic
      include ::Kernel

      abstract!

      ReturnType = type_member
      public_constant :ReturnType

      sig { returns(::T::Array[::CSVPlusPlus::Lexer::RaccToken]) }
      attr_reader :tokens

      sig { params(tokens: ::T::Array[::CSVPlusPlus::Lexer::RaccToken]).void }
      # Initialize a lexer instance with an empty +@tokens+
      def initialize(tokens: [])
        @tokens = ::T.let(tokens, ::T::Array[::CSVPlusPlus::Lexer::RaccToken])
      end

      sig { returns(::T.nilable(::CSVPlusPlus::Lexer::RaccToken)) }
      # Used by racc to iterate each token
      #
      # @return [Array<(Regexp, Symbol) | (false, false)>]
      def next_token
        @tokens.shift
      end

      sig { params(input: ::String).returns(::CSVPlusPlus::Lexer::RaccLexer::ReturnType) }
      # Orchestate the tokenizing, parsing and error handling of parsing input.  Each instance will implement their own
      # +#tokenizer+ method
      #
      # @return [RaccLexer#] Each instance will define it's own +return_value+ with the result of parsing
      # rubocop:disable Metrics/MethodLength
      def parse(input)
        return return_value unless anything_to_parse?(input)

        tokenize(input)
        do_parse
        return_value
      rescue ::Racc::ParseError => e
        raise(
          ::CSVPlusPlus::Error::FormulaSyntaxError.new(
            "Error parsing #{parse_subject}",
            bad_input: e.message,
            wrapped_error: e
          )
        )
      end
      # rubocop:enable Metrics/MethodLength

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

      sig { abstract.returns(::CSVPlusPlus::Lexer::RaccLexer::ReturnType) }
      # The output of the parser
      def return_value; end

      sig { abstract.returns(::CSVPlusPlus::Lexer::Tokenizer) }
      # Returns a +Lexer::Tokenizer+ configured for the given
      def tokenizer; end

      private

      sig { params(input: ::String).void }
      def tokenize(input)
        return if input.nil?

        t = tokenizer.scan(input)

        until t.scanner.empty?
          next if t.matches_ignore?

          return if t.stop?

          t.scan_tokens!
          consume_token(t)
        end

        @tokens << %i[EOL EOL]
      end

      sig { params(tokenizer: ::CSVPlusPlus::Lexer::Tokenizer).void }
      # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      def consume_token(tokenizer)
        if tokenizer.last_token&.token && tokenizer.last_match
          @tokens << [::T.must(tokenizer.last_token).token, ::T.must(tokenizer.last_match)]
        elsif tokenizer.scan_catchall
          @tokens << [::T.must(tokenizer.last_match), ::T.must(tokenizer.last_match)]
        # TODO: checking the +parse_subject+ like this is a little hacky... but we need to know if we're parsing
        # modifiers or code_section (or formulas in a cell)
        elsif parse_subject == 'modifier'
          raise(
            ::CSVPlusPlus::Error::ModifierSyntaxError.new(
              "Unable to parse #{parse_subject} starting at",
              bad_input: tokenizer.peek
            )
          )
        else
          raise(
            ::CSVPlusPlus::Error::FormulaSyntaxError.new(
              "Unable to parse #{parse_subject} starting at",
              bad_input: tokenizer.peek
            )
          )
        end
      end
      # rubocop:enable Metrics/AbcSize, Metrics/MethodLength
    end
  end
end
