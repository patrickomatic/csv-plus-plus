# typed: strict
# frozen_string_literal: true

require_relative './error/error'
require_relative './error/positional_error'

require_relative './error/cli_error'
require_relative './error/compiler_error'
require_relative './error/formula_syntax_error'
require_relative './error/modifier_syntax_error'
require_relative './error/modifier_validation_error'
require_relative './error/writer_error'

module CSVPlusPlus
  # A module containing errors to be raised
  module Error
  end
end
