# frozen_string_literal: true

module CSVPlusPlus
  module Language
    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF

    # A basic building block of the abstract syntax tree (AST)
    class Entity
      attr_reader :id, :type

      # initialize
      def initialize(type, id: nil)
        @type = type.to_sym
        @id = id.downcase.to_sym if id
      end

      # Is this a function-like entity?
      def function?
        false
      end

      # ==
      def ==(other)
        @type == other.type && @id == other.id
      end
    end

    # TODO: use mixins instead of inheritance for @id, @arguments logic
    ##
    # An entity that can take arguments
    class EntityWithArguments < Entity
      attr_reader :arguments

      # initialize
      def initialize(type, id: nil, arguments: [])
        super(type, id:)
        @arguments = arguments
      end

      # ==
      def ==(other)
        super || @arguments == other.arguments
      end

      # This entity is a function.  Worth distinction because it can have recursively
      # defined variables and forms the branching of our ASTs
      def function?
        true
      end

      protected

      attr_writer :arguments
    end

    ##
    # A static function definition
    class Function < EntityWithArguments
      attr_reader :body

      # initialize
      def initialize(id, arguments, body)
        super(:function, id:, arguments: arguments.map(&:to_sym))

        @body = body
      end

      # to_s
      def to_s
        @id.to_s.upcase
      end

      # ==
      def ==(other)
        super || (
          @body == other.body && @id == other.id
        )
      end
    end

    ##
    # A function call
    class FunctionCall < EntityWithArguments
      # initialize
      def initialize(id, arguments)
        super(:function_call, id:, arguments:)
      end

      # to_s
      def to_s
        @id.to_s.upcase
      end

      # ==
      def ==(other)
        super || @id == other.id
      end
    end

    ##
    # A reference to a cell
    class CellReference < Entity
      # initialize
      def initialize(id)
        super(:cell_reference, id:)
      end

      # to_s
      def to_s
        @id.to_s.upcase
      end

      # ==
      def ==(other)
        super || @id == other.id
      end
    end

    ##
    # A boolean value
    class Boolean < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:boolean)
        @value = value
      end

      # to_s
      def to_s
        @value.to_s.upcase
      end

      # ==
      def ==(other)
        super || value == other.value
      end
    end

    ##
    # A number value
    class Number < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:number)
        @value =
          if @value.is_a?(::String)
            value.include?('.') ? Float(value) : Integer(value, 10)
          else
            value
          end
      end

      # to_s
      def to_s
        @value.to_s
      end

      # ==
      def ==(other)
        super || value == other.value
      end
    end

    ##
    # A string value
    class String < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:string)
        @value = value.gsub(/^"|"$/, '')
      end

      # to_s
      def to_s
        "\"#{@value}\""
      end

      # ==
      def ==(other)
        super || value == other.value
      end
    end

    ##
    # A reference to a variable
    class Variable < Entity
      # initialize
      def initialize(id)
        super(:variable, id:)
      end

      # to_s
      def to_s
        "$$#{@id}"
      end

      # ==
      def ==(other)
        super || id == other.id
      end
    end

    ##
    # A runtime value
    #
    # These are values which can be materialized at any point via the +resolve_fn+
    # which takes an ExecutionContext as a param
    class RuntimeValue < Entity
      attr_reader :resolve_fn

      # initialize
      def initialize(resolve_fn)
        super(:runtime_variable)
        @resolve_fn = resolve_fn
      end

      # to_s
      def to_s
        @resolve_fn
      end
    end
  end
end
