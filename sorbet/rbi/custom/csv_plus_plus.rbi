# typed: strict

module CSVPlusPlus
  module Entities
    module ASTBuilder
      sig { params(value: T.any(String, T::Boolean)).returns(CSVPlusPlus::Entities::Boolean) }
      def boolean(value); end

      sig { params(value: String).returns(CSVPlusPlus::Entities::Date) }
      def date(value); end

      sig do
        params(
          id: Symbol, 
          arguments: T::Array[::Symbol], 
          body: CSVPlusPlus::Entities::Entity
        ).returns(CSVPlusPlus::Entities::Function)
      end
      def function(id, arguments, body); end

      sig { params(id: String, arguments: T::Array[CSVPlusPlus::Entities::Entity], infix: T::Boolean).void }
      def function_call(id, arguments, infix = false); end

      sig { params(value: T.any(String, Numeric)).returns(CSVPlusPlus::Entities::Number) }
      def number(value); end

      sig { params(ref: ::T.nilable(::String), a1_ref: ::T.nilable(::CSVPlusPlus::A1Reference)).void }
      def reference(ref: nil, a1_ref: nil); end

      sig { params(resolve_fn: CSVPlusPlus::Entities::RuntimeValue::ResolveFn).void }
      def runtime_value(resolve_fn); end

      sig { params(value: String).returns(CSVPlusPlus::Entities::String) }
      def string(value); end
    end
  end

  module Lexer
    module RaccLexer
      sig { void }
      def do_parse; end
    end
  end

  module Parser
    class CellValue
      sig { void }
      def initialize; end

      sig { params(input: ::String).returns(::String) }
      def parse(input); end
    end

    class CodeSection
      sig { params(scope: ::CSVPlusPlus::Runtime::Scope).void }
      def initialize(scope); end

      sig { params(input: ::String).returns(::String) }
      def parse(input); end
    end

    class Modifier
      sig { params(cell_modifier: CSVPlusPlus::Modifier::Modifier, row_modifier: CSVPlusPlus::Modifier::Modifier).void }
      def initialize(cell_modifier:, row_modifier:); end

      sig { params(input: ::String).returns(::String) }
      def parse(input); end
    end
  end
end

