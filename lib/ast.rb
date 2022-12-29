module GSPush
  class AST
    END_OF_CODE_SECTION = "---"
    VARIABLE_REF = "$$"

    def self.extract_variables(ast)
      
    end

    def self.interpolate_variables(ast, variables) 
      # XXX figure out the dependency to fill them
    end

    def self.dfs(ast, &block)
      if ast.first.is_a? Symbol
        yield ast
      else 
        node, rest = ast[0], ast[1...]
        yield node
        rest.each {|r| self.dfs(r, &block)}
      end
    end
  end
end
