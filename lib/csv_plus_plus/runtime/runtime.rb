# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # The runtime state of the compiler (the current +line_number+/+row_index+, +cell+ being processed, etc) for parsing
    # a given file.  We take multiple runs through the input file for parsing so it's really convenient to have a
    # central place for these things to be managed.
    #
    # @attr_reader position [Runtime::Position]
    # @attr_reader scope [Runtime::Scope]
    # @attr_reader source_code [SourceCode]
    class Runtime
      extend ::T::Sig

      sig { returns(::CSVPlusPlus::Runtime::Position) }
      attr_reader :position

      sig { returns(::CSVPlusPlus::Runtime::Scope) }
      attr_reader :scope

      sig { returns(::CSVPlusPlus::SourceCode) }
      attr_reader :source_code

      sig do
        params(
          source_code: ::CSVPlusPlus::SourceCode,
          position: ::T.nilable(::CSVPlusPlus::Runtime::Position),
          scope: ::T.nilable(::CSVPlusPlus::Runtime::Scope)
        ).void
      end
      # @param position [Position, nil] The (optional) position to start at
      # @param source_code [SourceCode] The source code being compiled
      # @param scope [Runtime::Scope, nil] The (optional) scope if it already exists
      def initialize(source_code:, position: nil, scope: nil)
        @source_code = source_code
        @scope = ::T.let(scope || ::CSVPlusPlus::Runtime::Scope.new, ::CSVPlusPlus::Runtime::Scope)
        @position = ::T.let(
          position || ::CSVPlusPlus::Runtime::Position.new(source_code.input),
          ::CSVPlusPlus::Runtime::Position
        )
      end

      sig { params(var_id: ::Symbol).returns(::CSVPlusPlus::Entities::CellReference) }
      # Bind +var_id+ to the current cell
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      #
      # @return [CellReference]
      def bind_variable_to_cell(var_id)
        ::CSVPlusPlus::Entities::CellReference.new(
          cell_index: position.cell_index,
          row_index: position.row_index
        ).tap do |var|
          scope.def_variable(var_id, var)
        end
      end

      sig do
        params(
          var_id: ::Symbol,
          expand: ::CSVPlusPlus::Modifier::Expand
        ).returns(::CSVPlusPlus::Entities::CellReference)
      end
      # Bind +var_id+ relative to an ![[expand]] modifier.
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      # @param expand [Expand] The expand where the variable is accessible (where it will be bound relative to)
      #
      # @return [CellReference]
      def bind_variable_in_expand(var_id, expand)
        ::CSVPlusPlus::Entities::CellReference.new(
          scoped_to_expand: expand,
          cell_index: position.cell_index
        ).tap do |var|
          scope.def_variable(var_id, var)
        end
      end

      sig { returns(::T::Boolean) }
      # Is the parser currently inside of the CSV section?
      #
      # @return [T::Boolean]
      def parsing_csv_section?
        source_code.in_csv_section?(position.line_number)
      end

      sig { params(ast: ::CSVPlusPlus::Entities::Entity).returns(::CSVPlusPlus::Entities::Entity) }
      # Resolve all values in the ast of the current cell being processed
      #
      # @param ast [Entities::Entity] The AST to replace references within
      #
      # @return [Entity] The AST with all references replaced
      # rubocop:disable Metrics/MethodLength
      def resolve_cell_value(ast)
        last_round = nil
        ::Kernel.loop do
          refs = ::CSVPlusPlus::Runtime::References.extract(ast, position, scope)
          return ast if refs.empty?

          # TODO: throw a +CompilerError+ here instead I think - basically we did a round and didn't make progress
          return ast if last_round == refs

          ast = scope.resolve_functions(
            position,
            scope.resolve_variables(position, ast, refs.variables),
            refs.functions
          )
        end
      end
      # rubocop:enable Metrics/MethodLength

      sig do
        type_parameters(:R).params(block: ::T.proc.returns(::T.type_parameter(:R))).returns(::T.type_parameter(:R))
      end
      # Each time we run a parse on the input, reset the runtime state starting at the beginning of the file
      # rubocop:disable Naming/BlockForwarding
      def start!(&block)
        position.line_number = 1
        position.start!(&block)
      end
      # rubocop:enable Naming/BlockForwarding

      sig do
        type_parameters(:R).params(block: ::T.proc.returns(::T.type_parameter(:R))).returns(::T.type_parameter(:R))
      end
      # Reset the runtime state starting at the CSV section
      # rubocop:disable Naming/BlockForwarding
      def start_at_csv!(&block)
        position.line_number = source_code.length_of_code_section + 1
        position.start!(&block)
      end
      # rubocop:enable Naming/BlockForwarding
    end
  end
end
