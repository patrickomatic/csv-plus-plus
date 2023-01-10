module CSVPlusPlus
  module Language
    protected

    class Entity
      attr_reader :type

      def id=(id)
        @id = id.downcase.to_sym
      end

      def ==(other)
        @type == other.type
      end
    end

    # TODO use mixins instead of inheritance for @id, @arguments logic
    class EntityWithArguments < Entity
      attr_reader :arguments

      def initialize(arguments)
        @arguments = arguments
      end

      def ==(other)
        super || @arguments == other.arguments
      end

      protected

      def arguments=(a)
        @arguments = a
      end
    end

    public

    class Function < EntityWithArguments
      attr_reader :body, :id

      def initialize(id, arguments, body)
        @type = :function
        @body = body

        self.id = id

        super arguments.map(&:to_sym)
      end

      def to_s
        @id.to_s.upcase
      end

      def ==(other)
        super || (
          self.body == body && 
          self.id == id
        )
      end
    end

    class FunctionCall < EntityWithArguments
      attr_reader :id

      def initialize(id, arguments)
        @type = :function_call
        @arguments = arguments

        self.id = id

        super arguments
      end

      def to_s
        @id.to_s.upcase
      end

      def ==(other)
        super || self.id == id
      end
    end

    class CellReference < Entity
      attr_reader :id

      def initialize(id)
        @type = :cell_reference
        self.id = id
      end

      def to_s
        @id.to_s.upcase
      end

      def ==(other)
        super || self.id == id
      end
    end

    class Boolean < Entity
      attr_reader :value

      def initialize(value)
        @type = :boolean
        @value = value
      end

      def to_s
        @value.to_s.upcase
      end

      def ==(other)
        super || self.value == value
      end
    end

    class Number < Entity
      attr_reader :value

      def initialize(value)
        @type = :number
        if @value.is_a? String
          @value = value.include?('.') ? value.to_f : value.to_i
        else
          @value = value
        end
      end

      def to_s
        @value.to_s
      end

      def ==(other)
        super || self.value == value
      end
    end

    class String < Entity
      attr_reader :value

      def initialize(value)
        @type = :string
        @value = value.gsub(/^"|"$/, '')
      end

      def to_s
        '"' + @value + '"'
      end

      def ==(other)
        super || self.value == value
      end
    end

    class Variable < Entity
      attr_reader :id

      def initialize(id)
        @type = :variable
        self.id = id
      end

      def to_s
        "$$#{@id.to_s}"
      end

      def ==(other)
        super || self.id == id
      end
    end

    class RuntimeValue < Entity
      attr_reader :resolve_fn

      def initialize(resolve_fn)
        @type = :runtime_variable
        @resolve_fn = resolve_fn
      end

      def to_s
        @resolve_fn
      end
    end
  end
end
