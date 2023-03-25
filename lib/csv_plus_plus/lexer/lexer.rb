# frozen_string_literal: true

require_relative '../color'

module CSVPlusPlus
  # Common methods to be mixed into the Racc parsers
  #
  # @attr_reader tokens [Array]
  module Lexer
    attr_reader :tokens

    # Initialize a lexer instance with an empty +@tokens+
    def initialize(tokens: [])
      @tokens = tokens
    end

    # Used by racc to iterate each token
    #
    # @return [Array<(String, String)>]
    def next_token
      @tokens.shift
    end

    # Orchestate the tokenizing, parsing and error handling of parsing input.  Each instance will implement their own
    #   #tokenizer method
    #
    # @return [Lexer#return_value] Each instance will define it's own +return_value+ with the result of parsing
    def parse(input, runtime)
      return if input.nil?

      return return_value unless anything_to_parse?(input)

      tokenize(input, runtime)
      do_parse
      return_value
    rescue ::Racc::ParseError => e
      runtime.raise_formula_syntax_error("Error parsing #{parse_subject}", e.message, wrapped_error: e)
    rescue ::CSVPlusPlus::Error::ModifierValidationError => e
      raise(::CSVPlusPlus::Error::ModifierSyntaxError.from_validation_error(runtime, e))
    end

    TOKEN_LIBRARY = {
      A1_NOTATION: [::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP, :A1_NOTATION],
      FALSE: [/false/i, :FALSE],
      HEX_COLOR: [::CSVPlusPlus::Color::HEX_STRING_REGEXP, :HEX_COLOR],
      ID: [/[$!\w:]+/, :ID],
      INFIX_OP: [%r{\^|\+|-|\*|/|&|<|>|<=|>=|<>}, :INFIX_OP],
      NUMBER: [/-?[\d.]+/, :NUMBER],
      STRING: [%r{"(?:[^"\\]|\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4}))*"}, :STRING],
      TRUE: [/true/i, :TRUE],
      VAR_REF: [/\$\$/, :VAR_REF]
    }.freeze
    public_constant :TOKEN_LIBRARY

    private

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

    def consume_token(tokenizer, runtime)
      if tokenizer.last_token
        @tokens << [tokenizer.last_token, tokenizer.last_match]
      elsif tokenizer.scan_catchall
        @tokens << [tokenizer.last_match, tokenizer.last_match]
      # TODO: checking the +parse_subject+ like this is a little hacky... but we need to know if we're parsing
      # modifiers or code_section (or formulas in a cell)
      elsif parse_subject == 'modifier'
        runtime.raise_modifier_syntax_error("Unable to parse #{parse_subject} starting at", tokenizer.peek)
      else
        runtime.raise_formula_syntax_error("Unable to parse #{parse_subject} starting at", tokenizer.peek)
      end
    end
  end
end
