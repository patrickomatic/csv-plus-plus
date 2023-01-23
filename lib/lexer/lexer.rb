# frozen_string_literal: true

require_relative '../language/syntax_error'
require 'strscan'

module CSVPlusPlus
  # Common methods to be mixed into our Racc parsers
  module Lexer
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

    protected

    def e(type, *entity_args)
      ::CSVPlusPlus::Language::TYPES[type].new(*entity_args)
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
