require 'global_scope'
require 'entities'

describe CSVPlusPlus::Language::GlobalScope do
  let(:ec) { build(:execution_context) }
  let(:code_section) { build(:code_section) }
  let(:global_scope) { described_class.new code_section }

  describe "#variable_references" do
    subject { global_scope.variable_references ast }

    context "without any references" do
      let(:ast) { build :number_one }
      it { is_expected.to eq [] }
    end

    context "with one reference" do
      let(:ast) { build :fn_call_foo }
      it { is_expected.to eq %i[bar] }
    end

    context "with deeply nested references" do
      let(:ast) do
        build(:fn_call, 
              a: build(:fn_call, 
                       a: build(:fn_call,
                                a: build(:variable_foo),
                                b: build(:number_one)),
                       b: build(:number_two)),
              b: build(:variable_bar))
      end
      it { is_expected.to eq(%i[foo bar]) }
    end

    context "with runtime references" do
      # XXX 
    end
  end

  describe "#resolve_static_variables" do
    let(:ast) { build(:fn_call, 
                      name: :multiply, 
                      a: build(:variable, id: :bar),
                      b: build(:variable, id: :foo)) }
    let(:variables) {
      {
        bar: build(:number_one),
        foo: build(:fn_call_add),
        dep: ast,
      }
    }

    subject { global_scope.resolve_static_variables(variables, execution_context: ec) }

    it "resolves the variables in dep" do
      expect(subject[:dep]).to eq build(:fn_call, 
                                        name: :multiply, 
                                        a: variables[:bar], 
                                        b: variables[:foo])
    end

    context "with runtime variables" do
      let(:ast) { build(:fn_call, 
                        name: :multiply, 
                        a: build(:variable, id: :rownum),
                        b: build(:variable, id: :foo)) }

      it "resolves just the static variables in dep" do
        expect(subject[:dep]).to eq build(:fn_call, 
                                          name: :multiply, 
                                          a: build(:variable, id: :rownum),
                                          b: variables[:foo])
      end
    end

    context "with undefined variables" do
      let(:variables) { { foo: build(:variable, id: :thisdoesnotexist) } }

      it "should raise a SyntaxError" do
        expect { subject }.to raise_error(CSVPlusPlus::Language::SyntaxError)
      end
    end
  end

  describe "#resolve_variable" do
    let(:ast) { build(:fn_call, 
                      name: :multiply,
                      a: build(:variable, id: :rownum),
                      b: build(:variable, id: :foo)) }

    subject { global_scope.resolve_variable(ast, :rownum, build(:number_one)) }

    it { is_expected.to eq build(:fn_call, 
                                 name: :multiply, 
                                 a: build(:number_one),
                                 b: build(:variable, id: :foo)) }

  end

  describe "#copy_tree_with_replacement" do
    let(:ast) do
      build(:fn_call, 
            a: build(:fn_call, 
                     a: build(:fn_call,
                              a: build(:variable_foo),
                              b: build(:number_one)),
                     b: build(:number_two)),
            b: build(:variable_bar))
    end

    subject { global_scope.copy_tree_with_replacement(ast, :fooz, :bar) }

    it { is_expected.to eq ast }

    it { is_expected.not_to be ast }
  end

  describe "#depth_first_search" do
    let(:number_5) { build(:number, n: 5) }
    let(:ast) { build(:fn_call, name: :multiply, a: number_5, b: number_5) }

    it "accumulates each value returned by the block" do
      expect(global_scope.depth_first_search(ast) {|n| 1 }).to eq [1, 1, 1]
    end

    context "with a number" do
      let(:ast) { build(:number_one) }

      it "yields the literal" do
        expect {|block|
          global_scope.depth_first_search(ast, &block)
        }.to yield_successive_args(ast)
      end
    end

    context "a function call" do
      it "yields the function and arguments in order" do
        expect {|block|
          global_scope.depth_first_search(ast, &block)
        }.to yield_successive_args(ast, number_5, number_5)
      end
    end
  end

  describe "#topological_sort" do
    let(:dependencies) { described_class::GraphHash[[
      ["a", ["b", "c"]],
      ["b", ["c"]],
      ["c", ["d"]],
      ["d", []]
    ]] }

    subject { global_scope.topological_sort(dependencies) }

    it "orders the keys by their required resolution order" do
      expect(subject).to eq(["d", "c", "b", "a"])
    end

    context "when it can't be resolved" do
      let(:dependencies) { described_class::GraphHash[[
        ["a", ["b"]],
        ["b", ["a"]],
      ]] }

      it "orders the keys by their required resolution order" do
        expect { subject }.to raise_error(TSort::Cyclic)
      end
    end
  end
end
