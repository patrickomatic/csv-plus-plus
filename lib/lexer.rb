# frozen_string_literal: true

require_relative 'language/syntax_error'
require 'strscan'

module CSVPlusPlus
  # A common lexer interface for use with racc gem
  module Lexer
    # A class that contains the use-case-specific regexes for parsing
    class Tokenizer
      attr_reader :catchall, :ignore, :scanner, :tokens

      # initialize
      def initialize(catchall:, ignore:, scanner:, tokens:)
        @catchall = catchall
        @ignore = ignore
        @tokens = tokens
        @scanner = scanner
      end

      # Scan tokens and see if any match
      def scan_tokens
        m = @tokens.find { |t| @scanner.scan(t.first) }
        m[1] if m
      end

      # Scan input against the catchall pattern
      def scan_catchall
        @scanner.scan(@catchall)
      end

      # Scan input against the ignore pattern
      def matches_ignore?
        @scanner.scan(@ignore)
      end
    end

    # initialize
    def initialize
      @tokens = []
    end

    # Used by racc to iterate each token
    def next_token
      @tokens.shift
    end

    # tokenize
    def tokenize(input, runtime)
      return if input.nil?

      s = ::StringScanner.new(input)
      t = tokenizer(s)

      until s.empty?
        next if t.matches_ignore?

        token = t.scan_tokens
        consume_token(token, s, t, runtime)
      end

      @tokens << %i[EOL EOL]
    end

    # parse
    def parse(input, runtime)
      return if input.nil? || !anything_to_parse?(input)

      tokenize(input, runtime)
      do_parse
      return_value
    rescue ::Racc::ParseError => e
      # XXX the name reference to cell value
      runtime.raise_syntax_error('Error parsing cell value', e.message, wrapped_error: e)
    end

    private

    def consume_token(token, scanner, tokenizer, runtime)
      if token
        @tokens << [token, scanner.matched]
      elsif tokenizer.scan_catchall
        @tokens << [scanner.matched, scanner.matched]
      else
        # TODO: naming reference
        runtime.raise_syntax_error('Unable to parse cell value starting at', s.peek(100))
      end
    end
  end
end
