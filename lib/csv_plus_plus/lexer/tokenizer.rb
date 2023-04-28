# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Lexer
    # A class that contains the use-case-specific regexes for parsing
    #
    # @attr_reader last_token [String, nil] The last token that's been matched.
    class Tokenizer
      extend ::T::Sig

      sig { returns(::T.nilable(::CSVPlusPlus::Lexer::Token)) }
      attr_reader :last_token

      sig do
        params(
          tokens: ::T::Enumerable[::CSVPlusPlus::Lexer::Token],
          catchall: ::T.nilable(::Regexp),
          ignore: ::T.nilable(::Regexp),
          alter_matches: ::T::Hash[::Symbol, ::T.proc.params(s: ::String).returns(::String)],
          stop_fn: ::T.nilable(::T.proc.params(s: ::StringScanner).returns(::T::Boolean))
        ).void
      end
      # @param tokens [Array<Regexp, String>] The list of tokens to scan
      # @param catchall [Regexp] A final regexp to try if nothing else matches
      # @param ignore [Regexp] Ignore anything matching this regexp
      # @param alter_matches [Object] A map of matches to alter
      # @param stop_fn [Proc] Stop parsing when this is true
      def initialize(tokens:, catchall: nil, ignore: nil, alter_matches: {}, stop_fn: nil)
        @last_token = ::T.let(nil, ::T.nilable(::CSVPlusPlus::Lexer::Token))

        @catchall = catchall
        @ignore = ignore
        @tokens = tokens
        @stop_fn = stop_fn
        @alter_matches = alter_matches
      end

      sig { params(input: ::String).returns(::T.self_type) }
      # Initializers a scanner for the given input to be parsed
      #
      # @param input The input to be tokenized
      #
      # @return [Tokenizer]
      def scan(input)
        @scanner = ::T.let(::StringScanner.new(input.strip), ::T.nilable(::StringScanner))
        self
      end

      sig { returns(::StringScanner) }
      # Returns the currently initialized +StringScanner+.  You must call +#scan+ first or else this will throw an
      # exception.
      #
      # @return [StringScanner]
      def scanner
        # The caller needs to initialize this class with a call to #scan before we can do anything.  it sets up the
        # +@scanner+ with it's necessary input.
        unless @scanner
          raise(::CSVPlusPlus::Error::CompilerError, 'Called Tokenizer#scanner without calling #scan first')
        end

        @scanner
      end

      sig { void }
      # Scan tokens and set +@last_token+ if any match
      #
      # @return [String, nil]
      def scan_tokens!
        @last_token = @tokens.find { |t| scanner.scan(t.regexp) }
      end

      sig { returns(::T.nilable(::String)) }
      # Scan input against the catchall pattern
      #
      # @return [String, nil]
      def scan_catchall
        scanner.scan(@catchall) if @catchall
      end

      sig { returns(::T.nilable(::String)) }
      # Scan input against the ignore pattern
      #
      # @return [boolean]
      def matches_ignore?
        scanner.scan(@ignore) if @ignore
      end

      sig { returns(::T.nilable(::String)) }
      # The value of the last token matched
      #
      # @return [String, nil]
      def last_match
        # rubocop:disable Style/MissingElse
        if @last_token && @alter_matches.key?(@last_token.token.to_sym)
          # rubocop:enable Style/MissingElse
          return ::T.must(@alter_matches[@last_token.token.to_sym]).call(scanner.matched)
        end

        scanner.matched
      end

      sig { params(peek_characters: ::Integer).returns(::String) }
      # Read the input but don't consume it
      #
      # @param peek_characters [Integer]
      #
      # @return [String]
      def peek(peek_characters: 100)
        scanner.peek(peek_characters)
      end

      sig { returns(::T::Boolean) }
      # Scan for our stop token (if there is one - some parsers stop early and some don't)
      #
      # @return [boolean]
      def stop?
        @stop_fn ? @stop_fn.call(scanner) : false
      end

      sig { returns(::String) }
      # The rest of the un-parsed input.  The tokenizer might not need to parse the entire input
      #
      # @return [String]
      def rest
        scanner.rest
      end
    end
  end
end
