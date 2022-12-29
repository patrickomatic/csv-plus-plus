require 'ast'

describe GSPush::AST do
  describe "::extract_variables" do
    # TODO
  end

  describe "::interpolate_variables" do
    # TODO 
  end

  describe "::dfs" do
    subject { GSPush::AST::dfs ast }

    describe "a literal" do
      let(:ast) { [:literal, "5"] }
      it "yields the literal" do
        expect {|block| 
          GSPush::AST::dfs(ast, &block)
        }.to yield_successive_args([:literal, "5"])
      end
    end

    describe "a function call" do
      let(:ast) { [[:fn, "MULTIPLY"], [[:literal, "5"], [:literal, "5"]]] }
      it "yields the function and arguments in order" do
        expect {|block| 
          GSPush::AST::dfs(ast, &block)
        }.to yield_successive_args([:fn, "MULTIPLY"], [:literal, "5"], [:literal, "5"])
      end
    end
  end
end
