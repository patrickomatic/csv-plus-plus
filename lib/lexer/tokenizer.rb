# frozen_string_literal: true

module CSVPlusPlus
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
  end
end
