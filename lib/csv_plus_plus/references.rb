# frozen_string_literal: true

require_relative 'graph'
require_relative 'scope'

module CSVPlusPlus
  # References in an AST that need to be resolved
  #
  # @attr functions [Array<Entities::Function>] Functions references
  # @attr variables [Array<Entities::Variable>] Variable references
  class References
    attr_accessor :functions, :variables

    # Extract references from an AST and return them in a new +References+ object
    #
    # @param ast [Entity] An +Entity+ to do a depth first search on for references.  Entities can be
    #   infinitely deep because they can contain other function calls as params to a function call
    # @param scope [Scope] The +CodeSection+ containing all currently defined functions & variables
    #
    # @return [References]
    def self.extract(ast, scope)
      new.tap do |refs|
        ::CSVPlusPlus::Graph.depth_first_search(ast) do |node|
          next unless node.function_call? || node.variable?

          refs.functions << node if function_reference?(node, scope)
          refs.variables << node if node.variable?
        end
      end
    end

    # Is the node a resolvable reference?
    #
    # @param node [Entity] The node to check if it's resolvable
    #
    # @return [boolean]
    # TODO: move this into the Entity subclasses
    def self.function_reference?(node, scope)
      node.function_call? && (scope.defined_function?(node.id) \
                              || ::CSVPlusPlus::Entities::Builtins::FUNCTIONS.key?(node.id))
    end

    private_class_method :function_reference?

    # Create an object with empty references.  The caller will build them up as it depth-first-searches
    def initialize
      @functions = []
      @variables = []
    end

    # Are there any references to be resolved?
    #
    # @return [boolean]
    def empty?
      @functions.empty? && @variables.empty?
    end

    # @return [String]
    def to_s
      "References(functions: #{@functions}, variables: #{@variables})"
    end

    # @return [boolean]
    def ==(other)
      @functions == other.functions && @variables == other.variables
    end
  end
end
