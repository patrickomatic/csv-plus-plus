# typed: strict

module CSVPlusPlus
  module Entities
    module ASTBuilder
      sig { params(value: T.any(String, T::Boolean)).returns(CSVPlusPlus::Entities::Boolean) }
      def boolean(value); end

      sig do
        params(
          cell_index: T.nilable(::Integer),
          ref: T.nilable(::String),
          row_index: T.nilable(::Integer),
          scoped_to_expand: T.nilable(::CSVPlusPlus::Modifier::Expand)
        ).returns(CSVPlusPlus::Entities::CellReference)
      end
      def cell_reference(cell_index: nil, ref: nil, row_index: nil, scoped_to_expand: nil); end

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

      sig { params(value: String).returns(CSVPlusPlus::Entities::String) }
      def string(value); end

      sig { params(id: Symbol).returns(CSVPlusPlus::Entities::Variable) }
      def variable(id); end
    end
  end

  module Parser
    class CellValue
      sig { void }
      def initialize; end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end

    class CodeSection
      sig { void }
      def initialize; end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end

    class Modifier
      sig { params(cell_modifier: CSVPlusPlus::Modifier::Modifier, row_modifier: CSVPlusPlus::Modifier::Modifier).void }
      def initialize(cell_modifier:, row_modifier:); end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end
  end
end

