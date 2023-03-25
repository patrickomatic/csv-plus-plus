# frozen_string_literal: true

require 'strscan'

module CSVPlusPlus
  module Lexer
    # A class that contains the use-case-specific regexes for parsing
    #
    # @attr_reader last_token [String] The last token that's been matched.
    # @attr_reader scanner [StringScanner] The StringScanner instance that's parsing the input.
    class Tokenizer
      attr_reader :last_token, :scanner

      # @param tokens [Array<Regexp, String>] The list of tokens to scan
      # @param catchall [Regexp] A final regexp to try if nothing else matches
      # @param ignore [Regexp] Ignore anything matching this regexp
      # @param alter_matches [Object] A map of matches to alter
      # @param stop_fn [Proc] Stop parsing when this is true
      def initialize(tokens:, catchall: nil, ignore: nil, alter_matches: {}, stop_fn: nil)
        @last_token = nil

        @catchall = catchall
        @ignore = ignore
        @tokens = tokens
        @stop_fn = stop_fn
        @alter_matches = alter_matches
      end

      # Initializers a scanner for the given input to be parsed
      #
      # @param input The input to be tokenized
      # @return [Tokenizer]
      def scan(input)
        @scanner = ::StringScanner.new(input.strip)
        self
      end

      # Scan tokens and set +@last_token+ if any match
      #
      # @return [String, nil]
      def scan_tokens!
        m = @tokens.find { |t| @scanner.scan(t.first) }
        @last_token = m ? m[1] : nil
      end

      # Scan input against the catchall pattern
      #
      # @return [String, nil]
      def scan_catchall
        @scanner.scan(@catchall) if @catchall
      end

      # Scan input against the ignore pattern
      #
      # @return [boolean]
      def matches_ignore?
        @scanner.scan(@ignore) if @ignore
      end

      # The value of the last token matched
      #
      # @return [String, nil]
      def last_match
        return @alter_matches[@last_token].call(@scanner.matched) if @alter_matches.key?(@last_token)

        @scanner.matched
      end

      # Read the input but don't consume it
      #
      # @param peek_characters [Integer]
      #
      # @return [String]
      def peek(peek_characters: 100)
        @scanner.peek(peek_characters)
      end

      # Scan for our stop token (if there is one - some parsers stop early and some don't)
      #
      # @return [boolean]
      def stop?
        @stop_fn ? @stop_fn.call(@scanner) : false
      end

      # The rest of the un-parsed input.  The tokenizer might not need to parse the entire input
      #
      # @return [String]
      def rest
        @scanner.rest
      end
    end
  end
end
