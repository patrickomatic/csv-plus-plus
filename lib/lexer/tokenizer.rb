# frozen_string_literal: true

require 'strscan'

module CSVPlusPlus
  module Lexer
    # A class that contains the use-case-specific regexes for parsing
    class Tokenizer
      attr_reader :last_token, :scanner

      # initialize
      # rubocop:disable Metrics/ParameterLists
      def initialize(input:, tokens:, catchall: nil, ignore: nil, alter_matches: {}, stop_fn: nil)
        @scanner = ::StringScanner.new(input.strip)
        @last_token = nil

        @catchall = catchall
        @ignore = ignore
        @tokens = tokens
        @stop_fn = stop_fn
        @alter_matches = alter_matches
      end
      # rubocop:enable Metrics/ParameterLists

      # Scan tokens and see if any match
      def scan_tokens!
        m = @tokens.find { |t| @scanner.scan(t.first) }
        @last_token = m ? m[1] : nil
      end

      # Scan input against the catchall pattern
      def scan_catchall
        @scanner.scan(@catchall) if @catchall
      end

      # Scan input against the ignore pattern
      def matches_ignore?
        @scanner.scan(@ignore) if @ignore
      end

      # The value of the last token matched
      def last_match
        return @alter_matches[@last_token].call(@scanner.matched) if @alter_matches.key?(@last_token)

        @scanner.matched
      end

      # Peek the input
      def peek
        @scanner.peek(100)
      end

      # Scan for our stop token (if there is one - some parsers stop early and some don't)
      def stop?
        @stop_fn ? @stop_fn.call(@scanner) : false
      end

      # The rest of the un-parsed input.  The tokenizer might not need to
      # parse the entire input
      def rest
        @scanner.rest
      end
    end
  end
end
