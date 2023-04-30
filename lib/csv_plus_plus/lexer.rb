# typed: strict
# frozen_string_literal: true

require_relative './lexer/racc_lexer'
require_relative './lexer/tokenizer'

module CSVPlusPlus
  # Code for tokenizing a csvpp file
  module Lexer
    extend ::T::Sig

    # A token that's matched by +regexp+ and presented with +token+
    class Token < ::T::Struct
      const :regexp, ::Regexp
      const :token, ::T.any(::String, ::Symbol)
    end

    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF

    # @see https://github.com/ruby/racc/blob/master/lib/racc/parser.rb#L121
    TOKEN_LIBRARY = ::T.let(
      {
        # A1_NOTATION: ::CSVPlusPlus::Lexer::Token.new(
        # regexp: ::CSVPlusPlus::A1Reference::A1_NOTATION_REGEXP, token: :A1_NOTATION
        # ),
        FALSE: ::CSVPlusPlus::Lexer::Token.new(regexp: /false/i, token: :FALSE),
        HEX_COLOR: ::CSVPlusPlus::Lexer::Token.new(regexp: ::CSVPlusPlus::Color::HEX_STRING_REGEXP, token: :HEX_COLOR),
        INFIX_OP: ::CSVPlusPlus::Lexer::Token.new(regexp: %r{\^|\+|-|\*|/|&|<|>|<=|>=|<>}, token: :INFIX_OP),
        NUMBER: ::CSVPlusPlus::Lexer::Token.new(regexp: /-?[\d.]+/, token: :NUMBER),
        REF: ::CSVPlusPlus::Lexer::Token.new(regexp: /[$!\w:]+/, token: :REF),
        STRING: ::CSVPlusPlus::Lexer::Token.new(
          regexp: %r{"(?:[^"\\]|\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4}))*"},
          token: :STRING
        ),
        TRUE: ::CSVPlusPlus::Lexer::Token.new(regexp: /true/i, token: :TRUE),
        VAR_REF: ::CSVPlusPlus::Lexer::Token.new(regexp: /\$\$/, token: :VAR_REF)
      }.freeze,
      ::T::Hash[::Symbol, ::CSVPlusPlus::Lexer::Token]
    )
    public_constant :TOKEN_LIBRARY

    sig { params(str: ::String).returns(::String) }
    # Run any transformations to the input before going into the CSV parser
    #
    # The CSV parser in particular does not like if there is whitespace after a double quote and before the next comma
    #
    # @param str [String]
    # @return [String]
    def self.preprocess(str)
      str.gsub(/"\s*,/, '",')
    end

    sig { params(str: ::String).returns(::String) }
    # When parsing a modifier with a quoted string field, we need a way to unescape.  Some examples of quoted and
    # unquoted results:
    #
    # * "just a string" => "just a string"
    # * "' this is a string'" => "this is a string"
    # * "won\'t this work?" => "won't this work"
    #
    # @param str [::String]
    #
    # @return [::String]
    def self.unquote(str)
      # could probably do this with one regex but we do it in 3 steps:
      #
      # 1. remove leading and trailing spaces and '
      # 2. remove any backslashes that are by themselves (none on either side)
      # 3. turn double backslashes into singles
      str.gsub(/^\s*'?|'?\s*$/, '').gsub(/([^\\]+)\\([^\\]+)/, '\1\2').gsub(/\\\\/, '\\')
    end
  end
end
