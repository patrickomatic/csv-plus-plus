# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Information about the unparsed source code
  class SourceCode
    extend ::T::Sig

    sig { returns(::String) }
    attr_reader :input

    sig { returns(::T.nilable(::Pathname)) }
    attr_reader :filename

    sig { returns(::Integer) }
    attr_reader :length_of_csv_section

    sig { returns(::Integer) }
    attr_reader :length_of_code_section

    sig { returns(::Integer) }
    attr_reader :length_of_file

    sig { params(input: ::String, filename: ::T.nilable(::String)).void }
    # @param input [::String] The source code being parsed
    # @param filename [::String, nil] The name of the file the source came from.  If not set we assume it came
    #   from stdin
    def initialize(input:, filename: nil)
      @input = input
      @filename = ::T.let(filename ? ::Pathname.new(filename) : nil, ::T.nilable(::Pathname))

      lines = input.split(/[\r\n]/)
      @length_of_file = ::T.let(lines.length, ::Integer)
      @length_of_code_section = ::T.let(count_code_section_lines(lines), ::Integer)
      @length_of_csv_section = ::T.let(@length_of_file - @length_of_code_section, ::Integer)
    end

    sig { params(line_number: ::Integer).returns(::T::Boolean) }
    # Does the given +line_number+ land in the code section of the file? (which includes the --- separator)
    #
    # @param line_number [Integer]
    #
    # @return [T::Boolean]
    def in_code_section?(line_number)
      line_number <= @length_of_code_section
    end

    sig { params(line_number: ::Integer).returns(::T::Boolean) }
    # Does the given +line_number+ land in the CSV section of the file?
    #
    # @param line_number [Integer]
    #
    # @return [T::Boolean]
    def in_csv_section?(line_number)
      line_number > @length_of_code_section
    end

    private

    sig { params(lines: ::T::Array[::String]).returns(::Integer) }
    def count_code_section_lines(lines)
      eoc = ::CSVPlusPlus::Lexer::END_OF_CODE_SECTION
      lines.include?(eoc) ? (lines.take_while { |l| l != eoc }).length + 1 : 0
    end
  end
end
