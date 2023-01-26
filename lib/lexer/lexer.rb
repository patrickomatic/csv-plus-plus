# frozen_string_literal: true

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

    # parse
    def parse(input, runtime)
      return if input.nil?

      return return_value unless anything_to_parse?(input)

      tokenize(input, runtime)
      do_parse
      return_value
    rescue ::Racc::ParseError => e
      runtime.raise_syntax_error("Error parsing #{parse_subject}", e.message, wrapped_error: e)
    end

    protected

    def tokenize(input, runtime)
      return if input.nil?

      t = tokenizer(input)

      until t.scanner.empty?
        next if t.matches_ignore?

        return if t.stop?

        t.scan_tokens!
        consume_token(t, runtime)
      end

      @tokens << %i[EOL EOL]
    end

    def e(type, *entity_args)
      ::CSVPlusPlus::Language::TYPES[type].new(*entity_args)
    end

    private

    def consume_token(tokenizer, runtime)
      if tokenizer.last_token
        @tokens << [tokenizer.last_token, tokenizer.last_match]
      elsif tokenizer.scan_catchall
        @tokens << [tokenizer.last_match, tokenizer.last_match]
      else
        runtime.raise_syntax_error("Unable to parse #{parse_subject} starting at", tokenizer.peek)
      end
    end
  end
end
