# typed: strict
# frozen_string_literal: true

require_relative './lexer/lexer'
require_relative './lexer/tokenizer'

module CSVPlusPlus
  # Code for tokenizing a csvpp file
  module Lexer
    extend ::T::Sig

    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF

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
